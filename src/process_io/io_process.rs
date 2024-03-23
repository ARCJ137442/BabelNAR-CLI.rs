//! 封装一个简单的「交互式输入输出」

use std::{
    ffi::OsStr,
    io::{BufRead, BufReader, Result as IoResult, Write},
    process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio, Termination},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};
use util::*;

/// 统一定义「输出侦听器」的类型
type OutputListener = dyn FnMut(String) + Send + Sync;

/// 构建一个「IO进程」
/// * 📌只是作为一个「构建器」存在
///   * 作为真正的`IoProcessManager`的launcher/builder
///
/// ! 因为有「系统指令」与「函数闭包」，无法派生任何常规宏
pub struct IoProcess {
    /// 内部封装的「进程指令」对象
    command: Command,
    /// 内部配置的「输出侦听器」
    out_listener: Option<Box<OutputListener>>,
}

impl IoProcess {
    /// 构造函数
    pub fn new(program_path: impl AsRef<OsStr>) -> Self {
        Self {
            command: Command::new(program_path),
            out_listener: None,
        }
    }

    /// 添加命令行参数
    pub fn arg(mut self, arg: impl AsRef<OsStr>) -> Self {
        // 添加参数
        self.command.arg(arg);
        // 返回自身以便链式调用
        self
    }

    /// 添加输出侦听器
    /// * 📌此处因生命周期问题（难以绑定`listener`到`self`）设置`F`的约束为`'static`
    pub fn out_listener<F>(mut self, listener: F) -> Self
    where
        F: FnMut(String) + Send + Sync + 'static,
    {
        // 字段赋值
        self.out_listener = Some(Box::new(listener));
        // 返回自身以便链式调用
        self
    }

    /// 启动
    /// * 🚩通过[`Self::try_launch`]尝试启动，然后直接解包
    ///
    /// # Panics
    /// * 📌如果子进程创建失败，将直接 panic
    pub fn launch(self) -> IoProcessManager {
        self
            // 尝试启动
            .try_launch()
            //解包
            .expect("无法启动子进程")
    }

    /// 启动
    /// * 🚩此处只负责创建子进程[`Child`]，
    ///   * ⚠️不负责对子进程的控制（监听、通道）等
    pub fn try_launch(mut self) -> std::io::Result<IoProcessManager> {
        // 创建一个子进程
        let child =
            // 指令+参数
            self.command
                .arg("shell")
                // 输入输出
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                // 产生进程
                .spawn()?;
        println!("Started process: {}", child.id());

        // 获取输出侦听器
        let out_listener = self.out_listener;

        // 创建「子进程管理器」对象
        Ok(IoProcessManager::new(child, out_listener))
    }
}

/// 子进程管理器
/// * 🎯负责
///   * 统一管理子进程
///   * 封装提供易用的（字符串）输入输出接口
/// * 🚩现在兼容「输出侦听」与「输出通道」两处
///   * 🎯「输出侦听」用于「需要**响应式**即时处理输出，但又不想阻塞主进程/开新进程」时
///   * 🎯「输出通道」用于「需要封装『并发异步获取』延迟处理输出，兼容已有异步并发模型」时
#[allow(dead_code)]
pub struct IoProcessManager {
    /// 正在管理的子进程
    process: Child,

    /// 子进程的「写（到子进程的）输入」守护线程
    thread_write_in: JoinHandle<()>,
    /// 子进程的「读（到子进程的）输出」守护线程
    /// * 🚩现在兼容「侦听器」「通道」两种模式，重新必要化
    // thread_read_out: Option<JoinHandle<()>>,
    thread_read_out: JoinHandle<()>,

    /// 子线程的终止信号
    termination_signal: Arc<Mutex<bool>>,

    /// 子进程输出的「接收者」
    /// * 🚩子进程发送给外部侦听器，由外部接收
    child_out: Mutex<Receiver<String>>,
    // ! 【2024-03-23 19:31:56】现在兼容「输出侦听」与「输出通道」二者
    /// 子进程输入的「发送者」
    /// * 🚩子进程接收来自外部发送的消息，由外部发送
    child_in: Mutex<Sender<String>>,
    // /// 子进程的「输出监听器」
    // out_listener: Option<Box<OutputListener>>,
    // ! 【2024-03-22 09:54:22】↑现在使用「输出侦听器」模式，此字段数据存储在`thread_read_out`中
}

impl IoProcessManager {
    // * 初始化 * //
    /// 构造方法
    /// * 🚩从「子进程」与「输出侦听器」构造「进程管理者」
    pub fn new(mut child: Child, out_listener: Option<Box<OutputListener>>) -> Self {
        // 提取子进程的标准输入输出
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();

        // 创建通道
        // * 📌IO流向：从左到右
        // ! 🚩【2024-03-22 09:53:12】现在采用「输出侦听器」的方法，不再需要封装通道
        let (child_out, out_sender) = channel();
        let (in_receiver, child_in) = channel();

        let termination_signal = Arc::new(Mutex::new(false));
        // 生成进程的「读写守护」（线程）
        let thread_write_in =
            IoProcessManager::spawn_thread_write_in(stdin, child_in, termination_signal.clone());
        let thread_read_out = IoProcessManager::spawn_thread_read_out(
            stdout,
            child_out,
            out_listener,
            termination_signal.clone(),
        );
        // let thread_read_out =
        // out_listener.map(|listener| IoProcessManager::spawn_thread_read_out(stdout, listener));
        // ! 🚩【2024-03-23 19:33:45】↑现在兼容「侦听器」「通道」二者

        // 捕获通道的两端
        let child_out_sender = Mutex::new(out_sender);
        let child_in_receiver = Mutex::new(in_receiver);

        // 构造并返回自身
        Self {
            process: child,
            thread_read_out,
            thread_write_in,
            child_out: child_out_sender,
            child_in: child_in_receiver,
            // out_listener,
            // ! 【2024-03-22 09:53:50】↑不再于自身存储「输出侦听器」，而是存储在`thread_read_out`中
            termination_signal,
        }
    }

    /// 生成一个子线程，管理子进程的标准输入，接收通道另一端输出
    /// * 📌读输入，写进程 | stdin >>> child_in_receiver
    #[inline]
    fn spawn_thread_write_in(
        stdin: ChildStdin,
        child_in_receiver: Receiver<String>,
        termination_signal: Arc<Mutex<bool>>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            // 从通道接收者读取输入 | 从「进程消息发送者」向进程发送文本
            let mut stdin = stdin;
            // ! 注意：这个`for`循环是阻塞的
            for line in child_in_receiver {
                // 写入输出
                if let Err(e) = stdin.write_all(line.as_bytes()) {
                    println!("无法向子进程输入：{e:?}");
                }
                // 检查终止信号
                if *termination_signal.lock().expect("无法锁定终止信号") {
                    // println!("子进程收到终止信号");
                    break;
                }
            }
        })
    }

    /// 生成一个子线程，管理子进程的标准输出，传送输出的消息到另一端
    /// * 📌写输出 | child_out_sender >>> stdout
    /// * 🚩【2024-03-23 20:46:38】现在「侦听器」与「通道」并行运作
    #[inline]
    fn spawn_thread_read_out(
        stdout: ChildStdout,
        child_out_sender: Sender<String>,
        out_listener: Option<Box<dyn FnMut(String) + Send + Sync>>,
        termination_signal: Arc<Mutex<bool>>,
    ) -> thread::JoinHandle<()> {
        // 将Option包装成一个新的函数
        // ! ⚠️【2024-03-23 19:54:43】↓类型注释是必须的：要约束闭包类型一致
        let mut listener_code: Box<dyn FnMut(&String) + Send + Sync> = match out_listener {
            // * 🚩先前有⇒实际执行 | 仅在实际有值时拷贝并传送给侦听器
            Some(mut listener) => Box::new(move |s: &String| listener(s.clone())),
            // * 🚩先前无⇒空函数
            None => Box::new(move |_| {}),
        };
        // 启动线程
        thread::spawn(move || {
            // 读取输出
            let mut stdout_reader = BufReader::new(stdout);
            let mut buf = String::new();
            // 持续循环
            loop {
                // 从子进程「标准输出」读取输入
                match stdout_reader.read_line(&mut buf) {
                    // 没有任何输入⇒检查终止信号
                    // * 📌不能在这里中断，需要检查终止信号
                    Ok(0) => {
                        if dbg!(*termination_signal.lock().expect("无法锁定终止信号")) {
                            // println!("子进程收到终止信号");
                            break;
                        }
                    }
                    // 有效输入
                    Ok(_) => {
                        // ! 🚩现在兼容「侦听器」「通道」二者
                        // 先侦听 | 只传递引用，仅在「实际有侦听器」时拷贝消息
                        listener_code(&buf);
                        // 向「进程消息接收者」传递消息（实际上是「输出」）
                        if let Err(e) = child_out_sender.send(buf.clone()) {
                            println!("无法接收子进程输出：{e:?}");
                            break;
                        }
                    }
                    // 报错⇒显示错误，终止读取
                    Err(e) => {
                        println!("子进程报错: {:?}", e);
                        break;
                    }
                }
                buf.clear();
            }
        })
    }

    // * 正常运作 * //

    /// 向子进程写入数据
    /// * 🚩通过使用自身「子进程输入」的互斥锁，从中输入数据
    /// * ⚠️返回空，或返回字符串形式的错误
    pub fn put(&self, input: impl ToString) -> Result<(), String> {
        // 从互斥锁中获取输入
        // * 🚩等待直到锁定互斥锁，最终在作用域结束（MutexGuard析构）时释放（解锁）
        let child_in_guard = self.child_in.lock().transform_err(|err| err.to_string())?;
        child_in_guard
            .send(input.to_string())
            .transform_err(|err| err.to_string())
    }

    /// 等待子进程结束
    /// * 🚩调用[`Child::wait`]方法
    /// * ⚠️对于【不会主动终止】的子进程，此举可能导致调用者死锁
    pub fn wait(&mut self) -> IoResult<ExitStatus> {
        self.process.wait()
    }

    /// 杀死自身
    /// * 🚩设置终止信号，通知子线程终止
    /// * 🚩调用[`Child::kill`]方法
    /// * ⚠️将终止自身
    pub fn kill(mut self) -> IoResult<()> {
        // ! ❌【2024-03-23 21:08:56】暂不独立其中的逻辑
        // 终止信号
        {
            *self.termination_signal.lock().unwrap() = true;
            dbg!(*self.termination_signal.lock().unwrap());
        }
        // 杀死子进程
        let kill_result = self.process.kill();
        dbg!(self.put("\n").unwrap()); // ! 解除子线程的阻塞

        // 等待子线程终止
        dbg!(self.thread_write_in.join().unwrap());
        dbg!(self.thread_read_out.join().unwrap());
        // 返回
        kill_result
        // self.process.kill()
    }

    /// 获取子进程id
    /// * 🚩调用[`Child::id`]方法
    pub fn id(&self) -> u32 {
        self.process.id()
    }
}

/// 单元测试
#[cfg(test)]
mod tests {

    use super::*;
    use std::{
        process::exit,
        sync::{Arc, Mutex},
        thread::sleep,
        time::Duration,
    };

    /// 测试/睡眠指定时间
    fn sleep_secs(secs: u64) {
        sleep(Duration::from_secs(secs));
    }

    // 定义一系列路径
    #[allow(unused)]
    const EXE_PATH_ONA: &str = r"..\..\NARS-executables\NAR.exe";
    #[allow(unused)]
    const EXE_PATH_REPL: &str = r"..\..\..\Julia\语言学小工Ju\繁简转换\dist\repl_简化.exe";
    #[allow(unused)]
    const EXE_PATH_ECHO: &str = r"..\NAVM.rs\target\debug\examples\echo_exe.exe";

    /// 标准案例：ONA交互
    ///
    /// ## 测试输入
    ///
    /// ```plaintext
    /// <A --> B>.
    /// <B --> C>.
    /// <A --> C>?
    /// ```
    ///
    /// ## 预期输出
    ///
    /// ```plaintext
    /// Answer: <A --> C>. creationTime=2 Truth: frequency=1.000000, confidence=0.810000
    /// ```
    ///
    /// ## 笔记
    ///
    /// * 📝[`Arc`]能满足[`Sync`]+[`Send`]，但R[`efCell`]不满足
    ///   * ❌无法使用`Arc<RefCell<T>>`组合
    /// * 📝[`Mutex`]能进行进程交互，但无法共享引用
    /// * 🚩最终使用`Arc<Mutex<T>>`作为进程交互的共享引用
    ///   * 📌[`Arc`]允许被拷贝并移动入闭包（共享引用，超越生命周期）
    ///   * 📌[`Mutex`]允许进程间共享的内部可变性（运行时借用检查）
    #[test]
    fn test_ona() {
        // 接收输出
        let outputs = Arc::new(Mutex::new(vec![]));
        let outputs_inner = outputs.clone();
        // 从一个系统指令开始构建并启动子进程
        let process = IoProcess::new(EXE_PATH_ONA)
            // 添加命令参数
            .arg("shell")
            // 添加输出监听器 | 简单回显
            // ! 【2024-03-22 10:06:38】基于「输出侦听器」的情形，若需要与外部交互，则会遇到所有权/生命周期问题
            // * 📄子进程与子进程外部（如此处的主进程）的问题
            // * ✅【2024-03-22 10:16:32】↑已使用`Arc<Mutex>`解决
            .out_listener(move |output: String| {
                outputs_inner
                    .lock()
                    .expect("无法锁定 outputs_inner")
                    .push(output.clone());
                print!("[OUT] {}", output);
            })
            // 启动子进程
            .launch();

        // 测试：输入输出 //
        let output_must_contains = |s: &str| {
            let outputs = outputs.lock().expect("无法锁定 outputs");
            let line = outputs
                .iter()
                .find(|line| line.contains(s))
                .expect("没有指定的输出！");
            println!("检验「{s:?}」成功！所在之处：{line:?}");
        };
        // 先置入输入
        sleep_secs(1);
        process.put("<A --> B>.\n").expect("无法放置输入");
        sleep_secs(1);

        // 中途检验
        output_must_contains("<A --> B>.");

        // 继续输入
        process.put("<B --> C>.\n").expect("无法放置输入");
        sleep_secs(1);
        process.put("<A --> C>?\n").expect("无法放置输入");
        sleep_secs(1);

        // 最后检验
        output_must_contains("Answer: <A --> C>.");

        // // 等待结束
        // process.wait();

        // 读取其中缓冲区里边的数据（多了会阻塞！）
        let r = process.child_out.lock().unwrap();
        for _ in 0..outputs.lock().unwrap().len() {
            let line = r.recv().expect("接收失败！");
            print!("从输出中读取到的一行（多了会阻塞！）：{line}");
        }

        // 等待2秒并强制结束
        println!("Waiting for 2 seconds and then killing the process...");
        sleep_secs(2);
        // process.kill().expect("无法杀死进程");
        println!("Process killed.");

        // 读取检验输出 | 杀死进程后还有
        dbg!(&*outputs);

        // 退出
        exit(0);
    }
}

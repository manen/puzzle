// use socketioxide::extract::Bin;

// pub trait BindFs {
// 	fn bind_fs<Fs: fs_trait::Fs>(&self, ns: &str, fs: Fs);
// }
// impl<A: socketioxide::adapter::Adapter> BindFs for socketioxide::SocketIo<A> {
// 	fn bind_fs<Fs: fs_trait::Fs>(&self, ns: &str, fs: Fs) {
// 		let on_connect = |socket: socketioxide::extract::SocketRef| async {
// 			socket.on(
// 				"message",
// 				|socket: socketioxide::extract::SocketRef, Bin(bin)| {
// 					fs.open(path)
// 				},
// 			);
// 		};

// 		self.ns(ns, on_connect);
// 	}
// }

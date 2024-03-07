use log::{Level, Log, Record};
use mixlayer::ByteBuffer;
use mixlayer_runtime_ffi::{
    prost::Message,
    protos::{VLog, VLogLevel},
};

extern "C" {
    fn _valence_log_v2(msg_ptr: *const ByteBuffer);
}

fn valence_log_v2(level: Level, target: String, message: String) {
    let level = match level {
        Level::Error => VLogLevel::LogLevelError,
        Level::Warn => VLogLevel::LogLevelWarn,
        Level::Info => VLogLevel::LogLevelInfo,
        Level::Debug => VLogLevel::LogLevelDebug,
        Level::Trace => VLogLevel::LogLevelTrace,
    } as i32;

    let proto = VLog {
        level,
        target,
        message,
    };

    let mut buf = vec![];
    proto.encode(&mut buf).unwrap();

    let buf = ByteBuffer::from_slice(&buf);

    unsafe {
        _valence_log_v2(&buf as *const ByteBuffer);
    };
}

struct ValenceLogger {
    level: Level,
}

impl Default for ValenceLogger {
    fn default() -> Self {
        Self { level: Level::Info }
    }
}

impl Log for ValenceLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            valence_log_v2(
                record.level(),
                record.target().to_owned(),
                format!("{}", record.args()),
            )
        }
    }

    fn flush(&self) {
        //no op
    }
}

pub fn init(level: log::Level) {
    let global_logger = Box::leak(Box::new(ValenceLogger { level }));

    log::set_logger(global_logger).unwrap();
    log::set_max_level(level.to_level_filter());
}

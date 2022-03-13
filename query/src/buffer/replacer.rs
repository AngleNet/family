use crate::config::FrameIdType;

pub trait Replacer {
	/// Evict the page held by the specified frame id.
	fn victim(&self) -> Option<FrameIdType>;

	fn pin(&mut self, frame_id: FrameIdType);

	fn unpin(&mut self, frame_id: FrameIdType);
}

pub struct LRUReplacer {}

impl Replacer for LRUReplacer {
	fn victim(&self) -> Option<FrameIdType> {
		todo!()
	}

	fn pin(&mut self, frame_id: FrameIdType) {
		todo!()
	}

	fn unpin(&mut self, frame_id: FrameIdType) {
		todo!()
	}
}

pub struct ClockReplacer {}

impl Replacer for ClockReplacer {
	fn victim(&self) -> Option<FrameIdType> {
		todo!()
	}

	fn pin(&mut self, frame_id: FrameIdType) {
		todo!()
	}

	fn unpin(&mut self, frame_id: FrameIdType) {
		todo!()
	}
}

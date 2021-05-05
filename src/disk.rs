use std::fs::{File, OpenOptions};
use std::io::{self, SeekFrom};
use std::path::Path;

pub const PAGE_SIZE: usize = 4096;

pub struct PageId(pub u64);
pub struct DiskManager {
  // ヒープファイルのファイルディスリプタ
  heap_file: file,
  // 採番するページIDを決めるカウンタ
  next_page_id: u64,
}

impl DiskManager {
  // コンストラクタ
  pub fn new(heap_file: file) -> io:Result<Self> {
    let heap_file_size = heap_file.metadata()?.len();
    let next_page_id = heap_file_size / PAGE_SIZE as u64
    Ok(Self {
      heap_file,
      next_page_id,
    })
  }

  //ファイルパスを指定して開く
  pub fn open(heap_file_path: impl AsRef(Path)) -> io:Result<Self> {
    let heao_file = OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(heap_file_path)?;
    Self::new(heap_file);
  }

  pub fn write_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
    // オフセットを計算
    let offset = PAGE_SIZE as u64 * page_id.to_u64();
    // ページ先頭へシーク
    self.heap_file.seek(SeekForm::Start(offset))?;
    // データを書き込む
    self.heap_file.write_all(data)
  }

pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
    // オフセットを計算
    let offset = PAGE_SIZE as u64 * page_id.to_u64();
    // ページ先頭へシーク
    self.heap_file.seek(SeekForm::Start(offset))?;
    // データを書き込む
    self.heap_file.read_exact(data)
}

  pub fn allocate_page(&mut self) -> PageId {
    let page_id =self.next_page_id;
    self.next_page_id += 1;
    PageId(page_id)
  }

}
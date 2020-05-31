use std::collections::HashSet;
use crate::response::TorrentInfo;

#[derive(Debug)]
pub struct SearchRequest {
    pub search: String,
    pub page: usize,
    pub order: Option<Order>,
    pub categories: HashSet<Category>,
}

#[derive(Debug)]
pub struct FileInfoRequest {
    pub id: u32,
}

#[derive(Debug)]
pub struct TorrentRequest {
    pub filename: String,
    pub id: u32,
}

impl SearchRequest {
    pub fn new(search: String) -> Self {
        Self {
            search: search,
            page: 1,
            order: None,
            categories: HashSet::new(),
        }
    }

    pub fn with_search(mut self, search: String) -> Self {
        self.search = search;
        self
    }

    pub fn with_page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }

    pub fn with_order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    pub fn with_category(mut self, category: Category) -> Self {
        if !self.categories.contains(&category) {
            self.categories.insert(category);
        }
        self
    }

    pub fn url(&self) -> String {
        let mut url = format!("https://www.iptorrents.com/t?{}&q={};p={}",
            self.category_string(),
            &self.search,
            self.page,
        );
        if let Some(o) = self.order {
            url.push_str(";");
            url.push_str(o.as_str());
        }
        url
    }

    fn category_string(&self) -> String {
        let mut s: String = String::new();
        self.categories
            .iter()
            .enumerate()
            .for_each(|(i, c)| {
                s.push_str(c.as_str());
                if i < self.categories.len() - 1 {
                    s.push_str(";");
                }
            });
        s
    }
}

impl FileInfoRequest {
    pub fn new(id: u32) -> Self {
        Self {
            id: id,
        }
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn url(&self) -> String {
        format!("https://www.iptorrents.com/t/{}/files",
            self.id,
        )
    }
}

impl From<&TorrentInfo> for FileInfoRequest {
    fn from(value: &TorrentInfo) -> Self {
        Self {
            id: value.id,
        }
    }
}

impl TorrentRequest {
    pub fn new(filename: String, id: u32) -> Self {
        Self {
            filename: filename,
            id: id,
        }
    }

    pub fn with_filename(mut self, filename: String) -> Self {
        self.filename = filename;
        self
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn url(&self) -> String {
        format!("https://www.iptorrents.com/download.php/{}/{}",
            self.id,
            self.filename,
        )
    }
}

impl From<&TorrentInfo> for TorrentRequest {
    fn from(value: &TorrentInfo) -> Self {
        Self {
            filename: value.torrent_title
                .to_string(),
            id: value.id,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Order {
	CommentCount,
	FileCount,
	Snatches,
	Leechers,
	Seeders,
	Name,
	Size,
}

impl Order {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CommentCount => "comments",
            Self::FileCount => "files-count",
            Self::Snatches => "completed",
            Self::Leechers => "leechers",
            Self::Seeders => "seeders",
            Self::Name => "name",
            Self::Size => "size",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Category {
	Movies,
	Movie3D,
	Movie480p,
	Movie4K,
	MovieBDR,
	MovieBDRip,
	MovieCam,
	MovieDVDR,
	MovieHDBluray,
	MovieKids,
	MovieMP4,
	MovieNonEnglish,
	MoviePacks,
	MovieWebDL,
	Moviex265,
	MovieXvid,
	TV,
	Documentaries,
	Sports,
	TV480p,
	TVBD,
	TVDVDR,
	TVDVDRip,
	TVMobile,
	TVNonEnglish,
	TVPacks,
	TVPacksNonEnglish,
	TVSDx264,
	TVWebDL,
	TVx264,
	TVx265,
	TVXvid,
	Games,
	GamesMixed,
	GamesNintendo,
	GamesPCISO,
	GamesPCRip,
	GamesPlaystation,
	GamesWii,
	GamesXbox,
	Music,
	MusicAudio,
	MusicFlac,
	MusicPacks,
	MusicVideo,
	Podcast,
	Miscellaneous,
	Anime,
	Appz,
	AppzNonEnglish,
	AudioBook,
	Books,
	Comics,
	Educational,
	Fonts,
	Mac,
	MagazinesNewspapers,
	Mobile,
	PicsWallpapers,
	Bookmarks,
	Subscriptions,
	Freeleech,
	New,
	StaffPicks,
	TopOfTheDay,
	TopOfTheWeek,
	TopOfTheMonth,
	TopOfTheQuarter,
	TopOfTheYear,
	_720P,
	_1080P,
	_2160P,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Movies => "72",
            Self::Movie3D => "87",
            Self::Movie480p => "77",
            Self::Movie4K => "101",
            Self::MovieBDR => "89",
            Self::MovieBDRip => "90",
            Self::MovieCam => "96",
            Self::MovieDVDR => "6",
            Self::MovieHDBluray => "48",
            Self::MovieKids => "54",
            Self::MovieMP4 => "62",
            Self::MovieNonEnglish => "38",
            Self::MoviePacks => "68",
            Self::MovieWebDL => "20",
            Self::Moviex265 => "100",
            Self::MovieXvid => "7",
            Self::TV => "73",
            Self::Documentaries => "26",
            Self::Sports => "55",
            Self::TV480p => "78",
            Self::TVBD => "23",
            Self::TVDVDR => "24",
            Self::TVDVDRip => "25",
            Self::TVMobile => "66",
            Self::TVNonEnglish => "82",
            Self::TVPacks => "65",
            Self::TVPacksNonEnglish => "83",
            Self::TVSDx264 => "79",
            Self::TVWebDL => "22",
            Self::TVx264 => "5",
            Self::TVx265 => "99",
            Self::TVXvid => "4",
            Self::Games => "74",
            Self::GamesMixed => "2",
            Self::GamesNintendo => "47",
            Self::GamesPCISO => "43",
            Self::GamesPCRip => "45",
            Self::GamesPlaystation => "71",
            Self::GamesWii => "50",
            Self::GamesXbox => "44",
            Self::Music => "75",
            Self::MusicAudio => "3",
            Self::MusicFlac => "80",
            Self::MusicPacks => "93",
            Self::MusicVideo => "37",
            Self::Podcast => "21",
            Self::Miscellaneous => "76",
            Self::Anime => "60",
            Self::Appz => "1",
            Self::AppzNonEnglish => "86",
            Self::AudioBook => "64",
            Self::Books => "35",
            Self::Comics => "94",
            Self::Educational => "95",
            Self::Fonts => "98",
            Self::Mac => "69",
            Self::MagazinesNewspapers => "92",
            Self::Mobile => "58",
            Self::PicsWallpapers => "36",
            Self::Bookmarks => "bookmarks",
            Self::Subscriptions => "subscriptions",
            Self::Freeleech => "free",
            Self::New => "new",
            Self::StaffPicks => "pinned",
            Self::TopOfTheDay => "top-of-the-day",
            Self::TopOfTheWeek => "top",
            Self::TopOfTheMonth => "top-of-the-month",
            Self::TopOfTheQuarter => "top-of-the-quarter",
            Self::TopOfTheYear => "top-of-the-year",
            Self::_720P => "720p",
            Self::_1080P => "1080p",
            Self::_2160P => "2160p",
        }
    }
}
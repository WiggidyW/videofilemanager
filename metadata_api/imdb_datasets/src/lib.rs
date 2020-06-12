mod streams;

#[derive(Debug, Clone, Copy)]
enum Dataset {
    NameBasics,
    TitleAkas,
    TitleBasics,
    TitleCrew,
    TitleEpisode,
    TitlePrincipals,
    TitleRatings,
}
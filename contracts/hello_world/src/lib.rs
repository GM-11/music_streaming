#![no_std]
use soroban_sdk::{ contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec };

#[contracttype]
pub struct Song {
    // pub title: Symbol,
    // pub artist: Symbol,
    // pub artist_id: Symbol,
    // pub uri: String,
    pub song_string: String,
}

#[contracttype]
pub struct Artist {
    pub id: u32,
    pub name: String,
}

pub const ALL_SONGS: Symbol = symbol_short!("songs");
pub const ALL_ARTISTS: Symbol = symbol_short!("artists");

#[contracttype]
pub struct SongsStruct {
    pub songs: Vec<Song>,
}
#[contracttype]
pub struct ArtistsStruct {
    pub songs: Vec<Artist>,
}

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn add_song(
        env: Env,
        song_string: String
        // title: Symbol,
        // artist: Symbol,
        // artist_id: Symbol,
        // uri: String
    ) -> Vec<Song> {
        let mut all_songs = env
            .storage()
            .instance()
            .get(&ALL_SONGS)
            .unwrap_or(SongsStruct { songs: Vec::new(&env) }).songs;

        all_songs.push_back(Song {
            song_string,
            // title,
            // artist,
            // artist_id,
            // uri,
        });

        env.storage().instance().set(&ALL_SONGS, &(SongsStruct { songs: all_songs.clone() }));

        env.storage().instance().extend_ttl(100, 100);

        all_songs
    }

    pub fn add_artist(env: Env, name: String) -> Vec<Artist> {
        let mut all_artists = env
            .storage()
            .instance()
            .get(&ALL_ARTISTS)
            .unwrap_or(ArtistsStruct { songs: Vec::new(&env) }).songs;

        if name != String::from_str(&env, "") {
            all_artists.push_back(Artist {
                id: all_artists.len() as u32,
                name,
            });

            env.storage()
                .instance()
                .set(&ALL_ARTISTS, &(ArtistsStruct { songs: all_artists.clone() }));

            env.storage().instance().extend_ttl(100, 100);
        }

        all_artists
    }

    pub fn get_songs(env: Env) -> Vec<Song> {
        let songs = env
            .storage()
            .instance()
            .get(&ALL_SONGS)
            .unwrap_or(SongsStruct { songs: Vec::new(&env) }).songs;

        env.storage().instance().extend_ttl(100, 100);

        songs
    }

    pub fn get_artists(env: Env) -> Vec<Artist> {
        let artists = env
            .storage()
            .instance()
            .get(&ALL_ARTISTS)
            .unwrap_or(ArtistsStruct { songs: Vec::new(&env) }).songs;

        env.storage().instance().extend_ttl(100, 100);

        artists
    }

    // pub fn get_song(env: Env, title: Symbol) -> Option<Song> {

    //     env.storage().instance().extend_ttl(100, 100);

    //     let all_songs = env
    //         .storage()
    //         .instance()
    //         .get(&ALL_SONGS)
    //         .unwrap_or(SongsStruct { songs: Vec::new(&env) }).songs;

    //     for song in all_songs {
    //         if song.title == title {
    //             return Some(song);
    //         }
    //     }

    //     None
    // }

    // pub fn get_artist(env: Env, id: Symbol) -> Option<Artist> {

    //     env.storage().instance().extend_ttl(100, 100);

    //     let all_artists = env
    //         .storage()
    //         .instance()
    //         .get(&ALL_ARTISTS)
    //         .unwrap_or(ArtistsStruct { songs: Vec::new(&env) }).songs;

    //     for artist in all_artists {
    //         if artist.id == id {
    //             return Some(artist);
    //         }
    //     }

    //     None
    // }
}

mod test;

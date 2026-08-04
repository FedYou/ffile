use ratatui::style::Color;

pub fn truncate_with_ellipsis(max_chars: usize, text: String) -> String {
    if text.chars().count() > max_chars {
        let truncated: String = text.chars().take(max_chars.saturating_sub(3)).collect();

        return format!("{}...", truncated.trim());
    }
    text
}

pub fn truncate_with_leading_ellipsis(max_chars: usize, text: String) -> String {
    let char_count = text.chars().count();
    if char_count > max_chars {
        let truncated: String = text
            .chars()
            .skip(char_count - max_chars.saturating_sub(3))
            .collect();

        return format!("...{}", truncated.trim());
    }
    text
}

pub fn get_sidebar_icon(name: &str) -> char {
    match name {
        "Home" => '\u{f02dc}',      // 󰋜
        "Desktop" => '\u{f01c5}',   // 󰇅
        "Downloads" => '\u{f01da}', // 󰇚
        "Pictures" => '\u{f03e}',   //
        "Music" => '\u{f0387}',     // 󰎇
        "Videos" => '\u{f52c}',     // 
        "Documents" => '\u{f1517}', // 󱔗
        _ => '\u{f07b}',            // 
    }
}

// Lista de iconos extradidos de exa: https://github.com/ogham/exa/blob/master/src/output/icons.rs

pub fn get_file_icon(file_name: &str, extension: &str, is_file: &bool) -> char {
    match file_name {
        ".Trash" => return '\u{f1f8}',             // 
        ".atom" => return '\u{e764}',              // 
        ".bashprofile" => return '\u{e615}',       // 
        ".bashrc" => return '\u{f489}',            // 
        ".git" => return '\u{f1d3}',               // 
        ".gitattributes" => return '\u{f1d3}',     // 
        ".gitconfig" => return '\u{f1d3}',         // 
        ".github" => return '\u{f09b}',            // 
        ".gitignore" => return '\u{f1d3}',         // 
        ".gitmodules" => return '\u{f1d3}',        // 
        ".rvm" => return '\u{e21e}',               // 
        ".vimrc" => return '\u{e62b}',             // 
        ".vscode" => return '\u{e70c}',            // 
        ".zshrc" => return '\u{f489}',             // 
        "Cargo.lock" => return '\u{e7a8}',         // 
        "bin" => return '\u{e5fc}',                // 
        "config" => return '\u{e5fc}',             // 
        "docker-compose.yml" => return '\u{f308}', // 
        "Dockerfile" => return '\u{f308}',         // 
        "ds_store" => return '\u{f179}',           // 
        "gitignore_global" => return '\u{f1d3}',   // 
        "go.mod" => return '\u{e626}',             // 
        "go.sum" => return '\u{e626}',             // 
        "gradle" => return '\u{e256}',             // 
        "gruntfile.coffee" => return '\u{e611}',   // 
        "gruntfile.js" => return '\u{e611}',       // 
        "gruntfile.ls" => return '\u{e611}',       // 
        "gulpfile.coffee" => return '\u{e610}',    // 
        "gulpfile.js" => return '\u{e610}',        // 
        "gulpfile.ls" => return '\u{e610}',        // 
        "include" => return '\u{e5fc}',            // 
        "lib" => return '\u{f121}',                // 
        "localized" => return '\u{f179}',          // 
        "Makefile" => return '\u{f489}',           // 
        "node_modules" => return '\u{e718}',       // 
        "npmignore" => return '\u{e71e}',          // 
        "PKGBUILD" => return '\u{f303}',           // 
        "rubydoc" => return '\u{e73b}',            // 
        "yarn.lock" => return '\u{e718}',          // 
        "LICENSE" | "LICENSE.md" | "LICENCE" | "LICENCE.md" | "copying" | "copying.md"
        | "copying.rst" | "copying.txt" | "copyright" | "copyright.md" | "copyright.rst"
        | "copyright.txt" | "license" | "license-agpl" | "license-apache" | "license-bsd"
        | "license-mit" | "license-gpl" | "license-lgpl" | "license.md" | "license.rst"
        | "license.txt" | "licence" | "licence-agpl" | "licence-apache" | "licence-bsd"
        | "licence-mit" | "licence-gpl" | "licence-lgpl" | "licence.md" | "licence.rst"
        | "licence.txt" => return '\u{f0fc3}', // 󰿃
        ".idea" => return '\u{e7b5}',              // 
        _ => {}
    };

    if !is_file {
        return '\u{f07b}'; // 
    }

    match extension {
        "ai" => '\u{e7b4}',             // 
        "android" => '\u{e70e}',        // 
        "apk" => '\u{e70e}',            // 
        "apple" => '\u{f179}',          // 
        "avi" => '\u{f03d}',            // 
        "avif" => '\u{f1c5}',           // 
        "avro" => '\u{e60b}',           // 
        "awk" => '\u{f489}',            // 
        "bash" => '\u{f489}',           // 
        "bash_history" => '\u{f489}',   // 
        "bash_profile" => '\u{f489}',   // 
        "bashrc" => '\u{f489}',         // 
        "bat" => '\u{f17a}',            // 
        "bats" => '\u{f489}',           // 
        "bmp" => '\u{f1c5}',            // 
        "bz" => '\u{f410}',             // 
        "bz2" => '\u{f410}',            // 
        "c" => '\u{e61e}',              // 
        "c++" => '\u{e61d}',            // 
        "cab" => '\u{e70f}',            // 
        "cc" => '\u{e61d}',             // 
        "cfg" => '\u{e615}',            // 
        "class" => '\u{e256}',          // 
        "clj" => '\u{e768}',            // 
        "cljs" => '\u{e76a}',           // 
        "cls" => '\u{f034}',            // 
        "cmd" => '\u{e70f}',            // 
        "coffee" => '\u{f0f4}',         // 
        "conf" => '\u{e615}',           // 
        "cp" => '\u{e61d}',             // 
        "cpio" => '\u{f410}',           // 
        "cpp" => '\u{e61d}',            // 
        "cs" => '\u{f031b}',            // 󰌛
        "csh" => '\u{f489}',            // 
        "cshtml" => '\u{f1fa}',         // 
        "csproj" => '\u{f031b}',        // 󰌛
        "css" => '\u{e749}',            // 
        "csv" => '\u{f1c3}',            // 
        "csx" => '\u{f031b}',           // 󰌛
        "cxx" => '\u{e61d}',            // 
        "d" => '\u{e7af}',              // 
        "dart" => '\u{e798}',           // 
        "db" => '\u{f1c0}',             // 
        "deb" => '\u{e77d}',            // 
        "diff" => '\u{f440}',           // 
        "djvu" => '\u{f02d}',           // 
        "dll" => '\u{e70f}',            // 
        "doc" => '\u{f1c2}',            // 
        "docx" => '\u{f1c2}',           // 
        "ds_store" => '\u{f179}',       // 
        "DS_store" => '\u{f179}',       // 
        "dump" => '\u{f1c0}',           // 
        "ebook" => '\u{e28b}',          // 
        "ebuild" => '\u{f30d}',         // 
        "editorconfig" => '\u{e615}',   // 
        "ejs" => '\u{e618}',            // 
        "elm" => '\u{e62c}',            // 
        "env" => '\u{f462}',            // 
        "eot" => '\u{f031}',            // 
        "epub" => '\u{e28a}',           // 
        "erb" => '\u{e73b}',            // 
        "erl" => '\u{e7b1}',            // 
        "ex" => '\u{e62d}',             // 
        "exe" => '\u{f17a}',            // 
        "exs" => '\u{e62d}',            // 
        "fish" => '\u{f489}',           // 
        "flac" => '\u{f001}',           // 
        "flv" => '\u{f03d}',            // 
        "font" => '\u{f031}',           // 
        "fs" => '\u{e7a7}',             // 
        "fsi" => '\u{e7a7}',            // 
        "fsx" => '\u{e7a7}',            // 
        "gdoc" => '\u{f1c2}',           // 
        "gem" => '\u{e21e}',            // 
        "gemfile" => '\u{e21e}',        // 
        "gemspec" => '\u{e21e}',        // 
        "gform" => '\u{f298}',          // 
        "gif" => '\u{f1c5}',            // 
        "git" => '\u{f1d3}',            // 
        "gitattributes" => '\u{f1d3}',  // 
        "gitignore" => '\u{f1d3}',      // 
        "gitmodules" => '\u{f1d3}',     // 
        "go" => '\u{e626}',             // 
        "gradle" => '\u{e256}',         // 
        "groovy" => '\u{e775}',         // 
        "gsheet" => '\u{f1c3}',         // 
        "gslides" => '\u{f1c4}',        // 
        "guardfile" => '\u{e21e}',      // 
        "gz" => '\u{f410}',             // 
        "h" => '\u{f0fd}',              // 
        "hbs" => '\u{e60f}',            // 
        "hpp" => '\u{f0fd}',            // 
        "hs" => '\u{e777}',             // 
        "htm" => '\u{f13b}',            // 
        "html" => '\u{f13b}',           // 
        "hxx" => '\u{f0fd}',            // 
        "ico" => '\u{f1c5}',            // 
        "image" => '\u{f1c5}',          // 
        "img" => '\u{e271}',            // 
        "iml" => '\u{e7b5}',            // 
        "ini" => '\u{f17a}',            // 
        "ipynb" => '\u{e678}',          // 
        "iso" => '\u{e271}',            // 
        "j2c" => '\u{f1c5}',            // 
        "j2k" => '\u{f1c5}',            // 
        "jad" => '\u{e256}',            // 
        "jar" => '\u{e256}',            // 
        "java" => '\u{e256}',           // 
        "jfi" => '\u{f1c5}',            // 
        "jfif" => '\u{f1c5}',           // 
        "jif" => '\u{f1c5}',            // 
        "jl" => '\u{e624}',             // 
        "jmd" => '\u{f48a}',            // 
        "jp2" => '\u{f1c5}',            // 
        "jpe" => '\u{f1c5}',            // 
        "jpeg" => '\u{f1c5}',           // 
        "jpg" => '\u{f1c5}',            // 
        "jpx" => '\u{f1c5}',            // 
        "js" => '\u{e74e}',             // 
        "json" => '\u{e60b}',           // 
        "jsx" => '\u{e7ba}',            // 
        "jxl" => '\u{f1c5}',            // 
        "ksh" => '\u{f489}',            // 
        "latex" => '\u{f034}',          // 
        "less" => '\u{e758}',           // 
        "lhs" => '\u{e777}',            // 
        "license" => '\u{f0219}',       // 󰈙
        "localized" => '\u{f179}',      // 
        "lock" => '\u{f033e}',          // 󰌾
        "log" => '\u{f18d}',            // 
        "lua" => '\u{e620}',            // 
        "lz" => '\u{f410}',             // 
        "lz4" => '\u{f410}',            // 
        "lzh" => '\u{f410}',            // 
        "lzma" => '\u{f410}',           // 
        "lzo" => '\u{f410}',            // 
        "m" => '\u{e61e}',              // 
        "mm" => '\u{e61d}',             // 
        "m4a" => '\u{f075a}',           // 󰝚
        "markdown" => '\u{f48a}',       // 
        "md" => '\u{f48a}',             // 
        "mjs" => '\u{e74e}',            // 
        "mk" => '\u{f489}',             // 
        "mkd" => '\u{f48a}',            // 
        "mkv" => '\u{f03d}',            // 
        "mobi" => '\u{e28b}',           // 
        "mov" => '\u{f03d}',            // 
        "mp3" => '\u{f075a}',           // 󰝚
        "mp4" => '\u{f03d}',            // 
        "msi" => '\u{e70f}',            // 
        "mustache" => '\u{e60f}',       // 
        "nix" => '\u{f313}',            // 
        "node" => '\u{f0399}',          // 󰎙
        "npmignore" => '\u{e71e}',      // 
        "odp" => '\u{f1c4}',            // 
        "ods" => '\u{f1c3}',            // 
        "odt" => '\u{f1c2}',            // 
        "ogg" => '\u{f075a}',           // 󰝚
        "ogv" => '\u{f03d}',            // 
        "otf" => '\u{f031}',            // 
        "part" => '\u{f43a}',           // 
        "patch" => '\u{f440}',          // 
        "pdf" => '\u{f1c1}',            // 
        "php" => '\u{e73d}',            // 
        "pl" => '\u{e769}',             // 
        "plx" => '\u{e769}',            // 
        "pm" => '\u{e769}',             // 
        "png" => '\u{f1c5}',            // 
        "pod" => '\u{e769}',            // 
        "ppt" => '\u{f1c4}',            // 
        "pptx" => '\u{f1c4}',           // 
        "procfile" => '\u{e21e}',       // 
        "properties" => '\u{e60b}',     // 
        "ps1" => '\u{f489}',            // 
        "psd" => '\u{e7b8}',            // 
        "pxm" => '\u{f1c5}',            // 
        "py" => '\u{e606}',             // 
        "pyc" => '\u{e606}',            // 
        "r" => '\u{f25d}',              // 
        "rakefile" => '\u{e21e}',       // 
        "rar" => '\u{f410}',            // 
        "razor" => '\u{f1fa}',          // 
        "rb" => '\u{e21e}',             // 
        "rdata" => '\u{f25d}',          // 
        "rdb" => '\u{e76d}',            // 
        "rdoc" => '\u{f48a}',           // 
        "rds" => '\u{f25d}',            // 
        "readme" => '\u{f48a}',         // 
        "rlib" => '\u{e7a8}',           // 
        "rmd" => '\u{f48a}',            // 
        "rpm" => '\u{e7bb}',            // 
        "rs" => '\u{e7a8}',             // 
        "rspec" => '\u{e21e}',          // 
        "rspec_parallel" => '\u{e21e}', // 
        "rspec_status" => '\u{e21e}',   // 
        "rss" => '\u{f09e}',            // 
        "rtf" => '\u{f0219}',           // 󰈙
        "ru" => '\u{e21e}',             // 
        "rubydoc" => '\u{e73b}',        // 
        "sass" => '\u{e603}',           // 
        "scala" => '\u{e737}',          // 
        "scss" => '\u{e749}',           // 
        "sh" => '\u{f489}',             // 
        "shell" => '\u{f489}',          // 
        "slim" => '\u{e73b}',           // 
        "sln" => '\u{e70c}',            // 
        "so" => '\u{f17c}',             // 
        "sql" => '\u{f1c0}',            // 
        "sqlite3" => '\u{e7c4}',        // 
        "sty" => '\u{f034}',            // 
        "styl" => '\u{e600}',           // 
        "stylus" => '\u{e600}',         // 
        "svg" => '\u{f1c5}',            // 
        "swift" => '\u{e755}',          // 
        "t" => '\u{e769}',              // 
        "tar" => '\u{f410}',            // 
        "taz" => '\u{f410}',            // 
        "tbz" => '\u{f410}',            // 
        "tbz2" => '\u{f410}',           // 
        "tex" => '\u{f034}',            // 
        "tgz" => '\u{f410}',            // 
        "tiff" => '\u{f1c5}',           // 
        "tlz" => '\u{f410}',            // 
        "toml" => '\u{e615}',           // 
        "torrent" => '\u{e275}',        // 
        "ts" => '\u{e628}',             // 
        "tsv" => '\u{f1c3}',            // 
        "tsx" => '\u{e7ba}',            // 
        "ttf" => '\u{f031}',            // 
        "twig" => '\u{e61c}',           // 
        "txt" => '\u{f15c}',            // 
        "txz" => '\u{f410}',            // 
        "tz" => '\u{f410}',             // 
        "tzo" => '\u{f410}',            // 
        "video" => '\u{f03d}',          // 
        "vim" => '\u{e62b}',            // 
        "vue" => '\u{f0844}',           // 󰡄
        "war" => '\u{e256}',            // 
        "wav" => '\u{f001}',            // 
        "webm" => '\u{f03d}',           // 
        "webp" => '\u{f1c5}',           // 
        "windows" => '\u{f17a}',        // 
        "woff" => '\u{f031}',           // 
        "woff2" => '\u{f031}',          // 
        "xhtml" => '\u{f13b}',          // 
        "xls" => '\u{f1c3}',            // 
        "xlsx" => '\u{f1c3}',           // 
        "xml" => '\u{f05c0}',           // 󰗀
        "xul" => '\u{f05c0}',           // 󰗀
        "xz" => '\u{f410}',             // 
        "yaml" => '\u{e8eb}',           // 
        "yml" => '\u{e8eb}',            // 
        "zip" => '\u{f410}',            // 
        "zsh" => '\u{f489}',            // 
        "zsh-theme" => '\u{f489}',      // 
        "zshrc" => '\u{f489}',          // 
        "zst" => '\u{f410}',            // 
        _ => '\u{f15b}',                // 
    }
}

pub fn get_file_icon_color(file_name: &str, extension: &str, is_file: &bool) -> Color {
    match file_name {
        ".Trash" => return Color::DarkGray,
        ".atom" => return Color::Green,
        ".bashprofile" => return Color::Green,
        ".bashrc" => return Color::Green,
        ".git" => return Color::Red,
        ".gitattributes" => return Color::Red,
        ".gitconfig" => return Color::Red,
        ".github" => return Color::White,
        ".gitignore" => return Color::Red,
        ".gitmodules" => return Color::Red,
        ".rvm" => return Color::Red,
        ".vimrc" => return Color::Green,
        ".vscode" => return Color::Blue,
        ".zshrc" => return Color::Green,
        "Cargo.lock" => return Color::Red,
        "bin" => return Color::Yellow,
        "config" => return Color::Yellow,
        "docker-compose.yml" => return Color::Blue,
        "Dockerfile" => return Color::Blue,
        "ds_store" => return Color::DarkGray,
        "gitignore_global" => return Color::Red,
        "go.mod" => return Color::Cyan,
        "go.sum" => return Color::Cyan,
        "gradle" => return Color::Blue,
        "gruntfile.coffee" => return Color::Yellow,
        "gruntfile.js" => return Color::Yellow,
        "gruntfile.ls" => return Color::Yellow,
        "gulpfile.coffee" => return Color::Red,
        "gulpfile.js" => return Color::Red,
        "gulpfile.ls" => return Color::Red,
        "include" => return Color::Yellow,
        "lib" => return Color::Yellow,
        "localized" => return Color::DarkGray,
        "Makefile" => return Color::Yellow,
        "node_modules" => return Color::Green,
        "npmignore" => return Color::Red,
        "PKGBUILD" => return Color::Blue,
        "rubydoc" => return Color::Red,
        "yarn.lock" => return Color::Cyan,
        ".idea" => return Color::Magenta,
        "LICENSE" | "LICENSE.md" | "LICENCE" | "LICENCE.md" | "copying" | "copying.md"
        | "copying.rst" | "copying.txt" | "copyright" | "copyright.md" | "copyright.rst"
        | "copyright.txt" | "license" | "license-agpl" | "license-apache" | "license-bsd"
        | "license-mit" | "license-gpl" | "license-lgpl" | "license.md" | "license.rst"
        | "license.txt" | "licence" | "licence-agpl" | "licence-apache" | "licence-bsd"
        | "licence-mit" | "licence-gpl" | "licence-lgpl" | "licence.md" | "licence.rst"
        | "licence.txt" => return Color::Red,
        _ => {}
    };

    // Directories
    if !is_file {
        return Color::Green;
    }

    // File extensions
    match extension {
        // Images
        "ai" => Color::Yellow,
        "avif" | "bmp" | "gif" | "ico" | "image" | "j2c" | "j2k" | "jfi" | "jfif" | "jif"
        | "jp2" | "jpe" | "jpeg" | "jpg" | "jpx" | "jxl" | "png" | "psd" | "pxm" | "svg"
        | "tiff" | "webp" => Color::Magenta,

        // Video
        "avi" | "flv" | "mkv" | "mov" | "mp4" | "ogv" | "video" | "webm" => Color::Blue,

        // Audio
        "flac" | "m4a" | "mp3" | "ogg" | "wav" => Color::Cyan,

        // Archives / compressed
        "bz" | "bz2" | "cpio" | "gz" | "lz" | "lz4" | "lzh" | "lzma" | "lzo" | "rar" | "tar"
        | "taz" | "tbz" | "tbz2" | "tgz" | "tlz" | "torrent" | "txz" | "tz" | "tzo" | "xz"
        | "zip" | "zst" => Color::Yellow,

        // Documents / text
        "doc" | "docx" | "gdoc" | "odt" | "rtf" => Color::Blue,
        "pdf" => Color::Red,
        "ppt" | "pptx" | "gslides" | "odp" => Color::Red,
        "xls" | "xlsx" | "csv" | "tsv" | "gsheet" | "ods" => Color::Green,
        "txt" | "readme" | "log" => Color::White,
        "md" | "markdown" | "mkd" | "rdoc" | "rmd" | "jmd" => Color::Blue,
        "epub" | "ebook" | "djvu" | "mobi" => Color::Cyan,
        "license" => Color::White,

        // Web / markup
        "html" | "htm" | "xhtml" => Color::Red,
        "css" => Color::Blue,
        "scss" => Color::Magenta,
        "sass" => Color::Magenta,
        "less" => Color::Blue,
        "styl" | "stylus" => Color::Green,
        "js" | "mjs" => Color::Yellow,
        "jsx" | "tsx" => Color::Cyan,
        "ts" => Color::Blue,
        "json" | "avro" | "properties" => Color::Yellow,
        "xml" | "xul" => Color::Yellow,
        "yaml" | "yml" => Color::Red,
        "toml" => Color::Yellow,
        "ini" | "cfg" | "conf" | "editorconfig" => Color::DarkGray,
        "env" => Color::Yellow,

        // Rust
        "rs" | "rlib" => Color::Red,

        // Go
        "go" => Color::Cyan,

        // Python
        "py" | "pyc" => Color::Yellow,

        // JavaScript / Node
        "node" => Color::Green,
        "npmignore" => Color::Red,

        // Ruby
        "rb" | "gem" | "gemfile" | "gemspec" | "guardfile" | "procfile" | "rakefile" | "rspec"
        | "rspec_parallel" | "rspec_status" | "ru" | "rubydoc" | "slim" | "erb" => Color::Red,

        // Java / JVM
        "java" | "class" | "jar" | "jad" | "war" => Color::Red,
        "gradle" => Color::Blue,
        "groovy" => Color::Blue,
        "scala" => Color::Red,
        "clj" | "cljs" => Color::Green,
        "kt" => Color::Magenta,

        // C / C++
        "c" | "m" => Color::Blue,
        "cpp" | "c++" | "cc" | "cp" | "cxx" | "mm" => Color::Blue,
        "h" | "hpp" | "hxx" => Color::Cyan,

        // C#
        "cs" | "csx" | "cshtml" | "csproj" | "razor" | "sln" => Color::Magenta,

        // Swift
        "swift" => Color::Red,

        // Kotlin
        "ksh" => Color::Green,

        // Shell / scripting
        "sh" | "shell" | "bash" | "bash_history" | "bash_profile" | "bashrc" | "bats" | "csh"
        | "fish" | "mk" | "ps1" | "zsh" | "zsh-theme" | "zshrc" | "awk" => Color::Green,
        "bat" | "cmd" | "exe" | "msi" | "cab" | "dll" | "windows" => Color::Blue,

        // Haskell / functional
        "hs" | "lhs" => Color::Magenta,
        "elm" => Color::Blue,
        "ex" | "exs" => Color::Magenta,
        "erl" => Color::Red,
        "jl" => Color::Magenta,
        "d" => Color::Red,
        "dart" => Color::Cyan,
        "lua" => Color::Blue,
        "r" | "rdata" | "rds" => Color::Blue,
        "perl" | "pl" | "plx" | "pm" | "pod" | "t" => Color::Blue,
        "php" => Color::Magenta,

        // F#
        "fs" | "fsi" | "fsx" => Color::Cyan,

        // Templates
        "hbs" | "mustache" | "twig" | "ejs" => Color::Yellow,
        "vue" => Color::Green,

        // Fonts
        "eot" | "font" | "otf" | "ttf" | "woff" | "woff2" => Color::Gray,

        // LaTeX
        "cls" | "latex" | "sty" | "tex" => Color::Green,

        // Database
        "db" | "dump" | "sql" | "sqlite3" => Color::Blue,
        "rdb" => Color::Red,

        // Diff / patch
        "diff" | "patch" => Color::Yellow,

        // Notebook
        "ipynb" => Color::Yellow,

        // Packaging / system
        "deb" | "rpm" => Color::Red,
        "ebuild" | "nix" => Color::Blue,
        "PKGBUILD" => Color::Blue,
        "apk" | "android" => Color::Green,
        "apple" => Color::White,
        "img" | "iso" => Color::DarkGray,
        "so" => Color::Yellow,

        // Misc
        "lock" => Color::DarkGray,
        "part" => Color::DarkGray,
        "rss" => Color::Yellow,
        "gform" => Color::Green,
        "iml" => Color::Magenta,
        "pem" | "cert" | "crt" | "key" => Color::Yellow,

        // Fallback
        _ => Color::Gray,
    }
}

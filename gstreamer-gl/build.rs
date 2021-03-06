fn main() {
    manage_docs();
}

#[cfg(all(
    any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"),
    not(all(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))
))]
fn manage_docs() {
    extern crate stripper_lib;
    use std::io;

    let path = "src";
    let ignores: &[&str] = &[];

    stripper_lib::loop_over_files(
        path.as_ref(),
        &mut |w, s| stripper_lib::strip_comments(w, s, &mut io::sink(), true),
        &ignores,
        false,
    );

    #[cfg(feature = "embed-lgpl-docs")]
    {
        let docs = include_str!("../docs/gstreamer-gl/docs.md");
        let mut infos = stripper_lib::parse_cmts(docs.lines(), true);
        stripper_lib::loop_over_files(
            path.as_ref(),
            &mut |w, s| stripper_lib::regenerate_comments(w, s, &mut infos, true, true),
            &ignores,
            false,
        );
    }
}

#[cfg(any(
    all(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"),
    not(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))
))]
fn manage_docs() {}

use bevy::{
    asset::{anyhow::Error, io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    reflect::TypePath,
    utils::BoxedFuture,
};
use rive_rs::{File, Instantiate};

#[derive(Asset, Deref, TypePath)]
pub struct Artboard(pub rive_rs::Artboard);

#[derive(Asset, TypePath)]
pub struct Riv(pub rive_rs::File);

#[derive(Default)]
pub struct RivLoader;

impl AssetLoader for RivLoader {
    type Asset = Riv;
    type Settings = ();
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let file = File::new(&bytes)?;

            let mut artboards = (0..)
                .into_iter()
                .map(|i| rive_rs::Artboard::instantiate(&file, Some(i)).map(|a| (i, a)));

            while let Some((i, artboard)) = artboards.next().flatten() {
                load_context.add_labeled_asset(format!("Artboard{}", i), Artboard(artboard));
            }

            Ok(Riv(file))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["riv"]
    }
}

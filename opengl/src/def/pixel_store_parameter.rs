pub enum PixelStoreParameter {
    UnpackSwapBytes = 3312,
    UnpackLsbFirst = 3313,
    UnpackRowLength = 3314,
    UnpackSkipRows = 3315,
    UnpackSkipPixels = 3316,
    UnpackAlignment = 3317,
    PackSwapBytes = 3328,
    PackLsbFirst = 3329,
    PackRowLength = 3330,
    PackSkipRows = 3331,
    PackSkipPixels = 3332,
    PackAlignment = 3333,
    PackSkipImages = 32875,
    PackImageHeight = 32876,
    UnpackSkipImages = 32877,
    UnpackImageHeight = 32878,
    PackSkipVolumesSgis = 33072,
    PackImageDepthSgis = 33073,
    UnpackSkipVolumesSgis = 33074,
    UnpackImageDepthSgis = 33075,
    PixelTileWidthSgix = 33088,
    PixelTileHeightSgix = 33089,
    PixelTileGridWidthSgix = 33090,
    PixelTileGridHeightSgix = 33091,
    PixelTileGridDepthSgix = 33092,
    PixelTileCacheSizeSgix = 33093,
    PackResampleSgix = 33838,
    UnpackResampleSgix = 33839,
    PackSubsampleRateSgix = 34208,
    UnpackSubsampleRateSgix = 34209,
    PackResampleOml = 35204,
    UnpackResampleOml = 35205,
}

#[allow(non_upper_case_globals)]
impl PixelStoreParameter {
    pub const UnpackRowLengthExt: PixelStoreParameter = PixelStoreParameter::UnpackRowLength;
    pub const UnpackSkipRowsExt: PixelStoreParameter = PixelStoreParameter::UnpackSkipRows;
    pub const UnpackSkipPixelsExt: PixelStoreParameter = PixelStoreParameter::UnpackSkipPixels;
    pub const PackSkipImagesExt: PixelStoreParameter = PixelStoreParameter::PackSkipImages;
    pub const PackImageHeightExt: PixelStoreParameter = PixelStoreParameter::PackImageHeight;
    pub const UnpackSkipImagesExt: PixelStoreParameter = PixelStoreParameter::UnpackSkipImages;
    pub const UnpackImageHeightExt: PixelStoreParameter = PixelStoreParameter::UnpackImageHeight;
}

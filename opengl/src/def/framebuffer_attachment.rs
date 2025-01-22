#[repr(u32)]
pub enum FramebufferAttachment {
    //
    // Summary:
    //     Original was GL_MAX_COLOR_ATTACHMENTS = 0x8CDF
    MaxColorAttachments = 36063,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT0 = 0x8CE0
    ColorAttachment0 = 36064,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT1 = 0x8CE1
    ColorAttachment1 = 36065,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT2 = 0x8CE2
    ColorAttachment2 = 36066,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT3 = 0x8CE3
    ColorAttachment3 = 36067,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT4 = 0x8CE4
    ColorAttachment4 = 36068,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT5 = 0x8CE5
    ColorAttachment5 = 36069,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT6 = 0x8CE6
    ColorAttachment6 = 36070,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT7 = 0x8CE7
    ColorAttachment7 = 36071,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT8 = 0x8CE8
    ColorAttachment8 = 36072,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT9 = 0x8CE9
    ColorAttachment9 = 36073,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT10 = 0x8CEA
    ColorAttachment10 = 36074,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT11 = 0x8CEB
    ColorAttachment11 = 36075,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT12 = 0x8CEC
    ColorAttachment12 = 36076,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT13 = 0x8CED
    ColorAttachment13 = 36077,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT14 = 0x8CEE
    ColorAttachment14 = 36078,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT15 = 0x8CEF
    ColorAttachment15 = 36079,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT16 = 0x8CF0
    ColorAttachment16 = 36080,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT17 = 0x8CF1
    ColorAttachment17 = 36081,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT18 = 0x8CF2
    ColorAttachment18 = 36082,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT19 = 0x8CF3
    ColorAttachment19 = 36083,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT20 = 0x8CF4
    ColorAttachment20 = 36084,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT21 = 0x8CF5
    ColorAttachment21 = 36085,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT22 = 0x8CF6
    ColorAttachment22 = 36086,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT23 = 0x8CF7
    ColorAttachment23 = 36087,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT24 = 0x8CF8
    ColorAttachment24 = 36088,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT25 = 0x8CF9
    ColorAttachment25 = 36089,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT26 = 0x8CFA
    ColorAttachment26 = 36090,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT27 = 0x8CFB
    ColorAttachment27 = 36091,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT28 = 0x8CFC
    ColorAttachment28 = 36092,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT29 = 0x8CFD
    ColorAttachment29 = 36093,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT30 = 0x8CFE
    ColorAttachment30 = 36094,
    //
    // Summary:
    //     Original was GL_COLOR_ATTACHMENT31 = 0x8CFF
    ColorAttachment31 = 36095,
    //
    // Summary:
    //     Original was GL_DEPTH_ATTACHMENT = 0x8D00
    DepthAttachment = 36096,
}

#[allow(non_upper_case_globals)]
impl FramebufferAttachment {
    pub const MaxColorAttachmentsExt: FramebufferAttachment =
        FramebufferAttachment::MaxColorAttachments;
    pub const MaxColorAttachmentsNv: FramebufferAttachment =
        FramebufferAttachment::MaxColorAttachments;

    pub const ColorAttachment0Ext: FramebufferAttachment = FramebufferAttachment::ColorAttachment0;
    pub const ColorAttachment0Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment0;
    pub const ColorAttachment0Oes: FramebufferAttachment = FramebufferAttachment::ColorAttachment0;

    pub const ColorAttachment1Ext: FramebufferAttachment = FramebufferAttachment::ColorAttachment1;
    pub const ColorAttachment1Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment1;

    pub const ColorAttachment2Ext: FramebufferAttachment = FramebufferAttachment::ColorAttachment2;
    pub const ColorAttachment2Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment2;

    pub const ColorAttachment3Ext: FramebufferAttachment = FramebufferAttachment::ColorAttachment3;
    pub const ColorAttachment3Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment3;

    pub const ColorAttachment4Ext: FramebufferAttachment = FramebufferAttachment::ColorAttachment4;
    pub const ColorAttachment4Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment4;

    pub const ColorAttachment5Ext: FramebufferAttachment = FramebufferAttachment::ColorAttachment5;
    pub const ColorAttachment5Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment5;

    pub const ColorAttachment6Ext: FramebufferAttachment = FramebufferAttachment::ColorAttachment6;
    pub const ColorAttachment6Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment6;

    pub const ColorAttachment7Ext: FramebufferAttachment = FramebufferAttachment::ColorAttachment7;
    pub const ColorAttachment7Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment7;

    pub const ColorAttachment8Ext: FramebufferAttachment = FramebufferAttachment::ColorAttachment8;
    pub const ColorAttachment8Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment8;

    pub const ColorAttachment9Ext: FramebufferAttachment = FramebufferAttachment::ColorAttachment9;
    pub const ColorAttachment9Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment9;

    pub const ColorAttachment10Ext: FramebufferAttachment =
        FramebufferAttachment::ColorAttachment10;
    pub const ColorAttachment10Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment10;

    pub const ColorAttachment11Ext: FramebufferAttachment =
        FramebufferAttachment::ColorAttachment11;
    pub const ColorAttachment11Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment11;

    pub const ColorAttachment12Ext: FramebufferAttachment =
        FramebufferAttachment::ColorAttachment12;
    pub const ColorAttachment12Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment12;

    pub const ColorAttachment13Ext: FramebufferAttachment =
        FramebufferAttachment::ColorAttachment13;
    pub const ColorAttachment13Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment13;

    pub const ColorAttachment14Ext: FramebufferAttachment =
        FramebufferAttachment::ColorAttachment14;
    pub const ColorAttachment14Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment14;

    pub const ColorAttachment15Ext: FramebufferAttachment =
        FramebufferAttachment::ColorAttachment15;
    pub const ColorAttachment15Nv: FramebufferAttachment = FramebufferAttachment::ColorAttachment15;

    pub const DepthAttachmentExt: FramebufferAttachment = FramebufferAttachment::DepthAttachment;
    pub const DepthAttachmentOes: FramebufferAttachment = FramebufferAttachment::DepthAttachment;
}

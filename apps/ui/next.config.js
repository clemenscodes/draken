// eslint-disable-next-line @typescript-eslint/no-var-requires
const { composePlugins, withNx } = require("@nx/next");

const isProd = process.env.NODE_ENV === "production";

module.exports = async (phase, { defaultConfig }) => {
    let internalHost = "";
    // In dev mode we use the internal-ip to serve the assets
    if (!isProd) {
        const { internalIpV4 } = await import("internal-ip");
        const ip = await internalIpV4();
        internalHost = ip ? ip : "";
    }
    const plugins = [withNx];
    /**
     * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
     **/
    const nextConfig = {
        nx: {
            svgr: false,
        },
        // Ensure Next.js uses SSG instead of SSR
        // https://nextjs.org/docs/pages/building-your-application/deploying/static-exports
        output: "export",
        // Note: This experimental feature is required to use NextJS Image in SSG mode.
        // See https://nextjs.org/docs/messages/export-image-api for different workarounds.
        images: {
            unoptimized: true,
        },
        // Configure assetPrefix or else the server won't properly resolve your assets.
        assetPrefix: isProd ? undefined : `http://${internalHost}:3000`,
    };
    return composePlugins(...plugins)(nextConfig)();
};

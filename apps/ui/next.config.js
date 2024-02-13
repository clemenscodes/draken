// eslint-disable-next-line @typescript-eslint/no-var-requires
const { composePlugins, withNx } = require("@nx/next");

const isProd = process.env.NODE_ENV === "production";

module.exports = async (phase, { defaultConfig }) => {
    let internalHost = "";
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
        output: "export",
        distDir: "../../dist/apps/ui",
        images: {
            unoptimized: true,
        },
        typescript: {
            ignoreBuildErrors: true,
        },
        assetPrefix: isProd ? undefined : `http://${internalHost}:3000`,
    };
    return composePlugins(...plugins)(nextConfig)();
};

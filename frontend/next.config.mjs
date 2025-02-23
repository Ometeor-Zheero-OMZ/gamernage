/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "export",
  reactStrictMode: false,
  images: {
    domains: ["localhost", "avatars.githubusercontent.com"],
    unoptimized: true,
  },
};

export default nextConfig;

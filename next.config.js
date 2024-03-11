/** @type {import('next').NextConfig} */
const nextConfig = {
  rewrites: async () => {
    const rewrites = {
      afterFiles: [
        // apply any of your existing rewrites here
      ],
      fallback: []
    }

    // dev only, this allows for local api calls to be proxied to
    // api routes that use rust runtime
    if (process.env.NODE_ENV === 'development') {
      rewrites.fallback.push({
        source: '/api/:path*',
        destination: 'http://0.0.0.0:3001/api/:path*'
      })
    }

    return rewrites
  }
}

module.exports = nextConfig

import typescript from '@rollup/plugin-typescript'

export default {
  input: 'guest-js/index.ts',
  output: [
    {
      file: 'dist-js/index.js',
      format: 'esm',
      sourcemap: true,
    },
    {
      file: 'dist-js/index.cjs',
      format: 'cjs',
      sourcemap: true,
    },
  ],
  plugins: [
    typescript({
      tsconfig: './tsconfig.json',
      declaration: true,
      declarationDir: './dist-js',
    }),
  ],
  external: ['@tauri-apps/api/core'],
}

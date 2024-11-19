/* eslint-disable import/no-extraneous-dependencies */
import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';
import eslintConfigPrettier from 'eslint-config-prettier';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { FlatCompat } from '@eslint/eslintrc';
import { fixupConfigRules } from '@eslint/compat';

const filename = fileURLToPath(import.meta.url);
const dirname = path.dirname(filename);
const compat = new FlatCompat({
  baseDirectory: dirname,
  recommendedConfig: fixupConfigRules(tseslint.config),
});

const airbnbTypescript = compat.extends('airbnb-typescript');
// Fix rules
const rule1 = airbnbTypescript[0].rules['@typescript-eslint/lines-between-class-members'];
airbnbTypescript[0].rules['@/lines-between-class-members'] = rule1;
delete airbnbTypescript[0].rules['@typescript-eslint/lines-between-class-members'];
delete airbnbTypescript[0].rules['@typescript-eslint/naming-convention'];
delete airbnbTypescript[0].rules['@typescript-eslint/dot-notation'];
delete airbnbTypescript[0].rules['@typescript-eslint/no-implied-eval'];
delete airbnbTypescript[0].rules['@typescript-eslint/no-throw-literal'];
delete airbnbTypescript[0].rules['@typescript-eslint/return-await'];

const config = [
  eslint.configs.recommended,
  ...fixupConfigRules(compat.extends('airbnb')),
  ...fixupConfigRules(airbnbTypescript),
  ...fixupConfigRules(compat.extends('airbnb/hooks')),
  eslintConfigPrettier,
  {
    ignores: ['node_modules', 'webapp/src/lib/wasm/', 'webapp/dist/', '**/*.config.*'],
  },
  {
    languageOptions: {
      parserOptions: {
        project: 'tsconfig.json',
        tsconfigRootDir: dirname,
      },
    },

    rules: {
      'react/require-default-props': 'off',
      'jsx-a11y/label-has-associated-control': [
        2,
        {
          assert: 'either',
        },
      ],
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          args: 'all',
          argsIgnorePattern: '^_',
          caughtErrors: 'all',
          caughtErrorsIgnorePattern: '^_',
          destructuredArrayIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          ignoreRestSiblings: true,
        },
      ],
      'import/extensions': [
        0,
        'ignorePackages',
      ]
    },
  },
  {
    "files": ["webapp/src/components/ui/*.tsx", "webapp/src/lib/utils.ts"],
    "rules": {
      "react/jsx-props-no-spreading": 0,
      "import/prefer-default-export": 0,
      "react/prop-types": 0,
    }
  }
];

export default config;

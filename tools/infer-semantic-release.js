// https://nx.dev/recipes/advanced-plugins/project-inference-plugins

const { normalizePath } = require('@nrwl/devkit');

const path = require('path');

function targetSemanticRelease(execCwd, overrideOptions = {}) {
  return {
    executor: '@theunderscorer/nx-semantic-release:semantic-release',
    options: {
      plugins: [
        [
          '@semantic-release/exec',
          {
            prepareCmd:
              'sleep 10 && cargo set-version ${nextRelease.version} && cargo check',
            publishCmd: 'cargo publish',
            execCwd,
          },
        ],
      ],
      ...overrideOptions,
    },
  };
}

exports.projectFilePatterns = ['Cargo.toml'];
exports.registerProjectTargets = function registerProjectTargets(
  projectFilePath
) {
  const execCwd = normalizePath(path.dirname(projectFilePath));

  return {
    test: {
      executor: 'nx:run-commands',
      options: {
        command: 'cargo test',
        color: true,
        cwd: execCwd,
      },
    },
    lint: {
      executor: 'nx:run-commands',
      options: {
        command: 'cargo clippy --no-deps -- --deny warnings',
        color: true,
        cwd: execCwd,
      },
    },
    'semantic-release': targetSemanticRelease(execCwd),
    'semantic-release-dry-run': targetSemanticRelease(execCwd, {
      dryRun: true,
    }),
  };
};

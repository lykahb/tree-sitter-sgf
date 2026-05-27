{
  "targets": [
    {
      "target_name": "tree_sitter_sgf_binding",
      "dependencies": [
        "<!(node -p \"require('node-addon-api').targets\"):node_addon_api"
      ],
      "include_dirs": [
        "src"
      ],
      "sources": [
        "bindings/node/binding.cc",
        "src/parser.c"
      ],
      "cflags_c": [
        "-std=c11"
      ],
      "cflags_cc": [
        "-std=c++17"
      ],
      "xcode_settings": {
        "OTHER_CFLAGS": [
          "-std=c11"
        ],
        "OTHER_CPLUSPLUSFLAGS": [
          "-std=c++17"
        ]
      },
      "msvs_settings": {
        "VCCLCompilerTool": {
          "AdditionalOptions": [
            "/std:c11",
            "/std:c++17",
            "/utf-8"
          ]
        }
      }
    }
  ]
}

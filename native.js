/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'blizt.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./blizt.android-arm64.node')
          } else {
            nativeBinding = require('blizt-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'blizt.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./blizt.android-arm-eabi.node')
          } else {
            nativeBinding = require('blizt-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'blizt.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./blizt.win32-x64-msvc.node')
          } else {
            nativeBinding = require('blizt-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'blizt.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./blizt.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('blizt-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'blizt.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./blizt.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('blizt-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'blizt.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./blizt.darwin-universal.node')
      } else {
        nativeBinding = require('blizt-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'blizt.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./blizt.darwin-x64.node')
          } else {
            nativeBinding = require('blizt-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'blizt.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./blizt.darwin-arm64.node')
          } else {
            nativeBinding = require('blizt-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'blizt.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./blizt.freebsd-x64.node')
      } else {
        nativeBinding = require('blizt-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'blizt.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./blizt.linux-x64-musl.node')
            } else {
              nativeBinding = require('blizt-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'blizt.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./blizt.linux-x64-gnu.node')
            } else {
              nativeBinding = require('blizt-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'blizt.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./blizt.linux-arm64-musl.node')
            } else {
              nativeBinding = require('blizt-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'blizt.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./blizt.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('blizt-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(
          join(__dirname, 'blizt.linux-arm-gnueabihf.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./blizt.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('blizt-linux-arm-gnueabihf')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'riscv64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'blizt.linux-riscv64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./blizt.linux-riscv64-musl.node')
            } else {
              nativeBinding = require('blizt-linux-riscv64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'blizt.linux-riscv64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./blizt.linux-riscv64-gnu.node')
            } else {
              nativeBinding = require('blizt-linux-riscv64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 's390x':
        localFileExisted = existsSync(
          join(__dirname, 'blizt.linux-s390x-gnu.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./blizt.linux-s390x-gnu.node')
          } else {
            nativeBinding = require('blizt-linux-s390x-gnu')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { BArray, BBoolean, BNull, BNumber, BObject, BString, BTuple, BUndefined, BUnion, BWrapped } = nativeBinding

module.exports.BArray = BArray
module.exports.BBoolean = BBoolean
module.exports.BNull = BNull
module.exports.BNumber = BNumber
module.exports.BObject = BObject
module.exports.BString = BString
module.exports.BTuple = BTuple
module.exports.BUndefined = BUndefined
module.exports.BUnion = BUnion
module.exports.BWrapped = BWrapped

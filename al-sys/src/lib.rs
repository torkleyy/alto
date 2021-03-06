#[cfg(not(target_os = "emscripten"))]
#[macro_use]
extern crate rental;


mod alc;
mod al;
mod efx;
mod efx_presets;

pub use alc::*;
pub use al::*;
pub use efx::*;
pub use efx_presets::*;


#[cfg(not(target_os = "emscripten"))]
macro_rules! al_api {
	{
		$($sym:ident: unsafe extern "C" fn ($($param:ident: $param_ty:ty),*) -> $ret_ty:ty,)*
	} => {
		mod al_api {
			extern crate libloading;

			use std::io;
			use std::path::Path;

			use super::*;


			#[allow(non_snake_case)]
			pub struct AlSymbols<'lib> {
				$(pub $sym: libloading::Symbol<'lib, unsafe extern "C" fn ($($param: $param_ty),*) -> $ret_ty>,)*
			}

			rental!{
				mod rent {
					use super::libloading;

					#[rental]
					pub struct RentSymbols {
						lib: Box<libloading::Library>,
						syms: super::AlSymbols<'lib>,
					}
				}
			}

			use self::rent::RentSymbols;


			pub struct AlApi(rent::RentSymbols);


			impl AlApi {
				pub fn load_default() -> io::Result<AlApi> {
					AlApi::from_lib(libloading::Library::new("libopenal.so")
						.or_else(|_| libloading::Library::new("libopenal.dylib"))
						.or_else(|_| libloading::Library::new("OpenAL.framework/OpenAL"))
						.or_else(|_| libloading::Library::new("soft_oal.dll"))
						.or_else(|_| libloading::Library::new("OpenAL32.dll"))
					?)
				}


				pub fn load<P: AsRef<Path>>(path: P) -> io::Result<AlApi> {
					AlApi::from_lib(libloading::Library::new(path.as_ref())?)
				}


				fn from_lib(lib: libloading::Library) -> io::Result<AlApi> {
					match RentSymbols::try_new(Box::new(lib), |lib| Ok(AlSymbols{
						$($sym: unsafe { lib.get(stringify!($sym).as_bytes())? },)*
					})) {
						Ok(syms) => Ok(AlApi(syms)),
						Err(rental::TryNewError(e, _)) => return Err(e),
					}
				}


				$(#[allow(non_snake_case)]
				#[inline]
				pub unsafe fn $sym(&self, $($param: $param_ty),*) -> $ret_ty {
					self.0.rent(|s| (s.$sym)($($param),*))
				})*
			}
		}

		pub use al_api::AlApi;
	};
}


#[cfg(target_os = "emscripten")]
macro_rules! al_api {
	{
		$($sym:ident: unsafe extern "C" fn ($($param:ident: $param_ty:ty),*) -> $ret_ty:ty,)*
	} => {
		mod al_api {
			use std::io;
			use std::path::Path;
			use super::*;


			#[allow(non_snake_case)]
			pub mod al_symbols {
				use super::*;

				extern "C" {
					$(pub fn $sym ($($param: $param_ty),*) -> $ret_ty;)*
				}
			}


			pub struct AlApi;


			impl AlApi {
				pub fn load_default() -> io::Result<AlApi> {
					Ok(AlApi)
				}


				pub fn load<P: AsRef<Path>>(_path: P) -> io::Result<AlApi> {
					Err(io::Error::new(io::ErrorKind::Other, "Dynamic loading is not supported on emscripten"))
				}


				$(#[allow(non_snake_case)]
				#[inline]
				pub unsafe fn $sym(&self, $($param: $param_ty),*) -> $ret_ty {
					al_symbols::$sym($($param),*)
				})*
			}
		}

		pub use al_api::AlApi;
	};
}


al_api! {
	alcCreateContext: unsafe extern "C" fn(device: *mut ALCdevice, attrlist: *const ALCint) -> *mut ALCcontext,
	alcMakeContextCurrent: unsafe extern "C" fn(context: *mut ALCcontext) -> ALCboolean,
	alcProcessContext: unsafe extern "C" fn(context: *mut ALCcontext) -> (),
	alcSuspendContext: unsafe extern "C" fn(context: *mut ALCcontext) -> (),
	alcDestroyContext: unsafe extern "C" fn(context: *mut ALCcontext) -> (),
	alcGetCurrentContext: unsafe extern "C" fn() -> *mut ALCcontext,
	alcGetContextsDevice: unsafe extern "C" fn(context: *mut ALCcontext) -> *mut ALCdevice,
	alcOpenDevice: unsafe extern "C" fn(devicename: *const ALCchar) -> *mut ALCdevice,
	alcCloseDevice: unsafe extern "C" fn(device: *mut ALCdevice) -> ALCboolean,
	alcGetError: unsafe extern "C" fn(device: *mut ALCdevice) -> ALCenum,
	alcIsExtensionPresent: unsafe extern "C" fn(device: *mut ALCdevice, extname: *const ALCchar) -> ALCboolean,
	alcGetProcAddress: unsafe extern "C" fn(device: *mut ALCdevice, funcname: *const ALCchar) -> *mut ::std::os::raw::c_void,
	alcGetEnumValue: unsafe extern "C" fn(device: *mut ALCdevice, enumname: *const ALCchar) -> ALCenum,
	alcGetString: unsafe extern "C" fn(device: *mut ALCdevice, param: ALCenum) -> *const ALCchar,
	alcGetIntegerv: unsafe extern "C" fn(device: *mut ALCdevice, param: ALCenum, size: ALCsizei, values: *mut ALCint) -> (),
	alcCaptureOpenDevice: unsafe extern "C" fn(devicename: *const ALCchar, frequency: ALCuint, format: ALCenum, buffersize: ALCsizei) -> *mut ALCdevice,
	alcCaptureCloseDevice: unsafe extern "C" fn(device: *mut ALCdevice) -> ALCboolean,
	alcCaptureStart: unsafe extern "C" fn(device: *mut ALCdevice) -> (),
	alcCaptureStop: unsafe extern "C" fn(device: *mut ALCdevice) -> (),
	alcCaptureSamples: unsafe extern "C" fn(device: *mut ALCdevice, buffer: *mut ALCvoid, samples: ALCsizei) -> (),

	alDopplerFactor: unsafe extern "C" fn(value: ALfloat) -> (),
	alDopplerVelocity: unsafe extern "C" fn(value: ALfloat) -> (),
	alSpeedOfSound: unsafe extern "C" fn(value: ALfloat) -> (),
	alDistanceModel: unsafe extern "C" fn(distanceModel: ALenum) -> (),
	alEnable: unsafe extern "C" fn(capability: ALenum) -> (),
	alDisable: unsafe extern "C" fn(capability: ALenum) -> (),
	alIsEnabled: unsafe extern "C" fn(capability: ALenum) -> ALboolean,
	alGetString: unsafe extern "C" fn(param: ALenum) -> *const ALchar,
	alGetBooleanv: unsafe extern "C" fn(param: ALenum, values: *mut ALboolean) -> (),
	alGetIntegerv: unsafe extern "C" fn(param: ALenum, values: *mut ALint) -> (),
	alGetFloatv: unsafe extern "C" fn(param: ALenum, values: *mut ALfloat) -> (),
	alGetDoublev: unsafe extern "C" fn(param: ALenum, values: *mut ALdouble) -> (),
	alGetBoolean: unsafe extern "C" fn(param: ALenum) -> ALboolean,
	alGetInteger: unsafe extern "C" fn(param: ALenum) -> ALint,
	alGetFloat: unsafe extern "C" fn(param: ALenum) -> ALfloat,
	alGetDouble: unsafe extern "C" fn(param: ALenum) -> ALdouble,
	alGetError: unsafe extern "C" fn() -> ALenum,
	alIsExtensionPresent: unsafe extern "C" fn(extname: *const ALchar) -> ALboolean,
	alGetProcAddress: unsafe extern "C" fn(fname: *const ALchar) -> *mut ::std::os::raw::c_void,
	alGetEnumValue: unsafe extern "C" fn(ename: *const ALchar) -> ALenum,
	alListenerf: unsafe extern "C" fn(param: ALenum, value: ALfloat) -> (),
	alListener3f: unsafe extern "C" fn(param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) -> (),
	alListenerfv: unsafe extern "C" fn(param: ALenum, values: *const ALfloat) -> (),
	alListeneri: unsafe extern "C" fn(param: ALenum, value: ALint) -> (),
	alListener3i: unsafe extern "C" fn(param: ALenum, value1: ALint, value2: ALint, value3: ALint) -> (),
	alListeneriv: unsafe extern "C" fn(param: ALenum, values: *const ALint) -> (),
	alGetListenerf: unsafe extern "C" fn(param: ALenum, value: *mut ALfloat) -> (),
	alGetListener3f: unsafe extern "C" fn(param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat) -> (),
	alGetListenerfv: unsafe extern "C" fn(param: ALenum, values: *mut ALfloat) -> (),
	alGetListeneri: unsafe extern "C" fn(param: ALenum, value: *mut ALint) -> (),
	alGetListener3i: unsafe extern "C" fn(param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint) -> (),
	alGetListeneriv: unsafe extern "C" fn(param: ALenum, values: *mut ALint) -> (),
	alGenSources: unsafe extern "C" fn(n: ALsizei, sources: *mut ALuint) -> (),
	alDeleteSources: unsafe extern "C" fn(n: ALsizei, sources: *const ALuint) -> (),
	alIsSource: unsafe extern "C" fn(source: ALuint) -> ALboolean,
	alSourcef: unsafe extern "C" fn(source: ALuint, param: ALenum, value: ALfloat) -> (),
	alSource3f: unsafe extern "C" fn(source: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) -> (),
	alSourcefv: unsafe extern "C" fn(source: ALuint, param: ALenum, values: *const ALfloat) -> (),
	alSourcei: unsafe extern "C" fn(source: ALuint, param: ALenum, value: ALint) -> (),
	alSource3i: unsafe extern "C" fn(source: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) -> (),
	alSourceiv: unsafe extern "C" fn(source: ALuint, param: ALenum, values: *const ALint) -> (),
	alGetSourcef: unsafe extern "C" fn(source: ALuint, param: ALenum, value: *mut ALfloat) -> (),
	alGetSource3f: unsafe extern "C" fn(source: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat) -> (),
	alGetSourcefv: unsafe extern "C" fn(source: ALuint, param: ALenum, values: *mut ALfloat) -> (),
	alGetSourcei: unsafe extern "C" fn(source: ALuint, param: ALenum, value: *mut ALint) -> (),
	alGetSource3i: unsafe extern "C" fn(source: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint) -> (),
	alGetSourceiv: unsafe extern "C" fn(source: ALuint, param: ALenum, values: *mut ALint) -> (),
	alSourcePlayv: unsafe extern "C" fn(n: ALsizei, sources: *const ALuint) -> (),
	alSourceStopv: unsafe extern "C" fn(n: ALsizei, sources: *const ALuint) -> (),
	alSourceRewindv: unsafe extern "C" fn(n: ALsizei, sources: *const ALuint) -> (),
	alSourcePausev: unsafe extern "C" fn(n: ALsizei, sources: *const ALuint) -> (),
	alSourcePlay: unsafe extern "C" fn(source: ALuint) -> (),
	alSourceStop: unsafe extern "C" fn(source: ALuint) -> (),
	alSourceRewind: unsafe extern "C" fn(source: ALuint) -> (),
	alSourcePause: unsafe extern "C" fn(source: ALuint) -> (),
	alSourceQueueBuffers: unsafe extern "C" fn(source: ALuint, nb: ALsizei, buffers: *const ALuint) -> (),
	alSourceUnqueueBuffers: unsafe extern "C" fn(source: ALuint, nb: ALsizei, buffers: *mut ALuint) -> (),
	alGenBuffers: unsafe extern "C" fn(n: ALsizei, buffers: *mut ALuint) -> (),
	alDeleteBuffers: unsafe extern "C" fn(n: ALsizei, buffers: *const ALuint) -> (),
	alIsBuffer: unsafe extern "C" fn(buffer: ALuint) -> ALboolean,
	alBufferData: unsafe extern "C" fn(buffer: ALuint, format: ALenum, data: *const ALvoid, size: ALsizei, freq: ALsizei) -> (),
	alBufferf: unsafe extern "C" fn(buffer: ALuint, param: ALenum, value: ALfloat) -> (),
	alBuffer3f: unsafe extern "C" fn(buffer: ALuint, param: ALenum, value1: ALfloat, value2: ALfloat, value3: ALfloat) -> (),
	alBufferfv: unsafe extern "C" fn(buffer: ALuint, param: ALenum, values: *const ALfloat) -> (),
	alBufferi: unsafe extern "C" fn(buffer: ALuint, param: ALenum, value: ALint) -> (),
	alBuffer3i: unsafe extern "C" fn(buffer: ALuint, param: ALenum, value1: ALint, value2: ALint, value3: ALint) -> (),
	alBufferiv: unsafe extern "C" fn(buffer: ALuint, param: ALenum, values: *const ALint) -> (),
	alGetBufferf: unsafe extern "C" fn(buffer: ALuint, param: ALenum, value: *mut ALfloat) -> (),
	alGetBuffer3f: unsafe extern "C" fn(buffer: ALuint, param: ALenum, value1: *mut ALfloat, value2: *mut ALfloat, value3: *mut ALfloat) -> (),
	alGetBufferfv: unsafe extern "C" fn(buffer: ALuint, param: ALenum, values: *mut ALfloat) -> (),
	alGetBufferi: unsafe extern "C" fn(buffer: ALuint, param: ALenum, value: *mut ALint) -> (),
	alGetBuffer3i: unsafe extern "C" fn(buffer: ALuint, param: ALenum, value1: *mut ALint, value2: *mut ALint, value3: *mut ALint) -> (),
	alGetBufferiv: unsafe extern "C" fn(buffer: ALuint, param: ALenum, values: *mut ALint) -> (),
}

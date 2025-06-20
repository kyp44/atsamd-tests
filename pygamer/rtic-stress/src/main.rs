//! A stress test of the RTC-based RTIC monotonic that uses over 768 concurrent tasks.
#![no_std]
#![no_main]

use hal::prelude::*;
use shared_pygamer::prelude::*;
use shared_pygamer::tests::async_stress::{self, test_task};

const BASE_PERIOD_MS: u32 = 1000;

pub async fn neopixels_task(neopixels: &mut NeoPixelsDriver, delay_ms: u32) -> ! {
    let mut colors = [
        RGB8::new(10, 0, 0),
        RGB8::new(0, 10, 0),
        RGB8::new(0, 0, 10),
    ]
    .into_iter()
    .cycle();

    loop {
        neopixels.write(colors.clone().take(5)).unwrap();

        colors.next().unwrap();

        Mono::delay_ms(delay_ms).await;
    }
}

#[rtic::app(device = pac, dispatchers = [EVSYS_0])]
mod app {
    use super::*;
    use bsp::RedLed;

    #[shared]
    struct Shared {
        display: DisplayDriver,
    }

    #[local]
    struct Local {
        red_led: RedLed,
        neopixels: NeoPixelsDriver,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut pkg = SetupPackage::new(cx.device, cx.core);
        let rtc = pkg.setup_rtc_clock().unwrap();

        // Start the monotonic
        Mono::general_start(pkg.delay.free(), rtc);

        // Display selected monotonic and clock
        display_monotonic_info(&mut pkg.display);

        #[cfg(feature = "neopixels")]
        test_neopixels::spawn().ok().unwrap();
        test_1::spawn().ok().unwrap();
        test_2::spawn().ok().unwrap();
        test_3::spawn().ok().unwrap();
        test_4::spawn().ok().unwrap();
        test_5::spawn().ok().unwrap();
        test_6::spawn().ok().unwrap();
        test_7::spawn().ok().unwrap();
        test_8::spawn().ok().unwrap();
        test_9::spawn().ok().unwrap();
        test_10::spawn().ok().unwrap();
        test_11::spawn().ok().unwrap();
        test_12::spawn().ok().unwrap();
        test_13::spawn().ok().unwrap();
        test_14::spawn().ok().unwrap();
        test_15::spawn().ok().unwrap();
        test_16::spawn().ok().unwrap();
        test_17::spawn().ok().unwrap();
        test_18::spawn().ok().unwrap();
        test_19::spawn().ok().unwrap();
        test_20::spawn().ok().unwrap();
        test_21::spawn().ok().unwrap();
        test_22::spawn().ok().unwrap();
        test_23::spawn().ok().unwrap();
        test_24::spawn().ok().unwrap();
        test_25::spawn().ok().unwrap();
        test_26::spawn().ok().unwrap();
        test_27::spawn().ok().unwrap();
        test_28::spawn().ok().unwrap();
        test_29::spawn().ok().unwrap();
        test_30::spawn().ok().unwrap();
        test_31::spawn().ok().unwrap();
        test_32::spawn().ok().unwrap();
        test_33::spawn().ok().unwrap();
        test_34::spawn().ok().unwrap();
        test_35::spawn().ok().unwrap();
        test_36::spawn().ok().unwrap();
        test_37::spawn().ok().unwrap();
        test_38::spawn().ok().unwrap();
        test_39::spawn().ok().unwrap();
        test_40::spawn().ok().unwrap();
        test_41::spawn().ok().unwrap();
        test_42::spawn().ok().unwrap();
        test_43::spawn().ok().unwrap();
        test_44::spawn().ok().unwrap();
        test_45::spawn().ok().unwrap();
        test_46::spawn().ok().unwrap();
        test_47::spawn().ok().unwrap();
        test_48::spawn().ok().unwrap();
        test_49::spawn().ok().unwrap();
        test_50::spawn().ok().unwrap();
        test_51::spawn().ok().unwrap();
        test_52::spawn().ok().unwrap();
        test_53::spawn().ok().unwrap();
        test_54::spawn().ok().unwrap();
        test_55::spawn().ok().unwrap();
        test_56::spawn().ok().unwrap();
        test_57::spawn().ok().unwrap();
        test_58::spawn().ok().unwrap();
        test_59::spawn().ok().unwrap();
        test_60::spawn().ok().unwrap();
        test_61::spawn().ok().unwrap();
        test_62::spawn().ok().unwrap();
        test_63::spawn().ok().unwrap();
        test_64::spawn().ok().unwrap();
        test_65::spawn().ok().unwrap();
        test_66::spawn().ok().unwrap();
        test_67::spawn().ok().unwrap();
        test_68::spawn().ok().unwrap();
        test_69::spawn().ok().unwrap();
        test_70::spawn().ok().unwrap();
        test_71::spawn().ok().unwrap();
        test_72::spawn().ok().unwrap();
        test_73::spawn().ok().unwrap();
        test_74::spawn().ok().unwrap();
        test_75::spawn().ok().unwrap();
        test_76::spawn().ok().unwrap();
        test_77::spawn().ok().unwrap();
        test_78::spawn().ok().unwrap();
        test_79::spawn().ok().unwrap();
        test_80::spawn().ok().unwrap();
        test_81::spawn().ok().unwrap();
        test_82::spawn().ok().unwrap();
        test_83::spawn().ok().unwrap();
        test_84::spawn().ok().unwrap();
        test_85::spawn().ok().unwrap();
        test_86::spawn().ok().unwrap();
        test_87::spawn().ok().unwrap();
        test_88::spawn().ok().unwrap();
        test_89::spawn().ok().unwrap();
        test_90::spawn().ok().unwrap();
        test_91::spawn().ok().unwrap();
        test_92::spawn().ok().unwrap();
        test_93::spawn().ok().unwrap();
        test_94::spawn().ok().unwrap();
        test_95::spawn().ok().unwrap();
        test_96::spawn().ok().unwrap();
        test_97::spawn().ok().unwrap();
        test_98::spawn().ok().unwrap();
        test_99::spawn().ok().unwrap();
        test_100::spawn().ok().unwrap();
        test_101::spawn().ok().unwrap();
        test_102::spawn().ok().unwrap();
        test_103::spawn().ok().unwrap();
        test_104::spawn().ok().unwrap();
        test_105::spawn().ok().unwrap();
        test_106::spawn().ok().unwrap();
        test_107::spawn().ok().unwrap();
        test_108::spawn().ok().unwrap();
        test_109::spawn().ok().unwrap();
        test_110::spawn().ok().unwrap();
        test_111::spawn().ok().unwrap();
        test_112::spawn().ok().unwrap();
        test_113::spawn().ok().unwrap();
        test_114::spawn().ok().unwrap();
        test_115::spawn().ok().unwrap();
        test_116::spawn().ok().unwrap();
        test_117::spawn().ok().unwrap();
        test_118::spawn().ok().unwrap();
        test_119::spawn().ok().unwrap();
        test_120::spawn().ok().unwrap();
        test_121::spawn().ok().unwrap();
        test_122::spawn().ok().unwrap();
        test_123::spawn().ok().unwrap();
        test_124::spawn().ok().unwrap();
        test_125::spawn().ok().unwrap();
        test_126::spawn().ok().unwrap();
        test_127::spawn().ok().unwrap();
        test_128::spawn().ok().unwrap();
        test_129::spawn().ok().unwrap();
        test_130::spawn().ok().unwrap();
        test_131::spawn().ok().unwrap();
        test_132::spawn().ok().unwrap();
        test_133::spawn().ok().unwrap();
        test_134::spawn().ok().unwrap();
        test_135::spawn().ok().unwrap();
        test_136::spawn().ok().unwrap();
        test_137::spawn().ok().unwrap();
        test_138::spawn().ok().unwrap();
        test_139::spawn().ok().unwrap();
        test_140::spawn().ok().unwrap();
        test_141::spawn().ok().unwrap();
        test_142::spawn().ok().unwrap();
        test_143::spawn().ok().unwrap();
        test_144::spawn().ok().unwrap();
        test_145::spawn().ok().unwrap();
        test_146::spawn().ok().unwrap();
        test_147::spawn().ok().unwrap();
        test_148::spawn().ok().unwrap();
        test_149::spawn().ok().unwrap();
        test_150::spawn().ok().unwrap();
        test_151::spawn().ok().unwrap();
        test_152::spawn().ok().unwrap();
        test_153::spawn().ok().unwrap();
        test_154::spawn().ok().unwrap();
        test_155::spawn().ok().unwrap();
        test_156::spawn().ok().unwrap();
        test_157::spawn().ok().unwrap();
        test_158::spawn().ok().unwrap();
        test_159::spawn().ok().unwrap();
        test_160::spawn().ok().unwrap();
        test_161::spawn().ok().unwrap();
        test_162::spawn().ok().unwrap();
        test_163::spawn().ok().unwrap();
        test_164::spawn().ok().unwrap();
        test_165::spawn().ok().unwrap();
        test_166::spawn().ok().unwrap();
        test_167::spawn().ok().unwrap();
        test_168::spawn().ok().unwrap();
        test_169::spawn().ok().unwrap();
        test_170::spawn().ok().unwrap();
        test_171::spawn().ok().unwrap();
        test_172::spawn().ok().unwrap();
        test_173::spawn().ok().unwrap();
        test_174::spawn().ok().unwrap();
        test_175::spawn().ok().unwrap();
        test_176::spawn().ok().unwrap();
        test_177::spawn().ok().unwrap();
        test_178::spawn().ok().unwrap();
        test_179::spawn().ok().unwrap();
        test_180::spawn().ok().unwrap();
        test_181::spawn().ok().unwrap();
        test_182::spawn().ok().unwrap();
        test_183::spawn().ok().unwrap();
        test_184::spawn().ok().unwrap();
        test_185::spawn().ok().unwrap();
        test_186::spawn().ok().unwrap();
        test_187::spawn().ok().unwrap();
        test_188::spawn().ok().unwrap();
        test_189::spawn().ok().unwrap();
        test_190::spawn().ok().unwrap();
        test_191::spawn().ok().unwrap();
        test_192::spawn().ok().unwrap();
        test_193::spawn().ok().unwrap();
        test_194::spawn().ok().unwrap();
        test_195::spawn().ok().unwrap();
        test_196::spawn().ok().unwrap();
        test_197::spawn().ok().unwrap();
        test_198::spawn().ok().unwrap();
        test_199::spawn().ok().unwrap();
        test_200::spawn().ok().unwrap();
        test_201::spawn().ok().unwrap();
        test_202::spawn().ok().unwrap();
        test_203::spawn().ok().unwrap();
        test_204::spawn().ok().unwrap();
        test_205::spawn().ok().unwrap();
        test_206::spawn().ok().unwrap();
        test_207::spawn().ok().unwrap();
        test_208::spawn().ok().unwrap();
        test_209::spawn().ok().unwrap();
        test_210::spawn().ok().unwrap();
        test_211::spawn().ok().unwrap();
        test_212::spawn().ok().unwrap();
        test_213::spawn().ok().unwrap();
        test_214::spawn().ok().unwrap();
        test_215::spawn().ok().unwrap();
        test_216::spawn().ok().unwrap();
        test_217::spawn().ok().unwrap();
        test_218::spawn().ok().unwrap();
        test_219::spawn().ok().unwrap();
        test_220::spawn().ok().unwrap();
        test_221::spawn().ok().unwrap();
        test_222::spawn().ok().unwrap();
        test_223::spawn().ok().unwrap();
        test_224::spawn().ok().unwrap();
        test_225::spawn().ok().unwrap();
        test_226::spawn().ok().unwrap();
        test_227::spawn().ok().unwrap();
        test_228::spawn().ok().unwrap();
        test_229::spawn().ok().unwrap();
        test_230::spawn().ok().unwrap();
        test_231::spawn().ok().unwrap();
        test_232::spawn().ok().unwrap();
        test_233::spawn().ok().unwrap();
        test_234::spawn().ok().unwrap();
        test_235::spawn().ok().unwrap();
        test_236::spawn().ok().unwrap();
        test_237::spawn().ok().unwrap();
        test_238::spawn().ok().unwrap();
        test_239::spawn().ok().unwrap();
        test_240::spawn().ok().unwrap();
        test_241::spawn().ok().unwrap();
        test_242::spawn().ok().unwrap();
        test_243::spawn().ok().unwrap();
        test_244::spawn().ok().unwrap();
        test_245::spawn().ok().unwrap();
        test_246::spawn().ok().unwrap();
        test_247::spawn().ok().unwrap();
        test_248::spawn().ok().unwrap();
        test_249::spawn().ok().unwrap();
        test_250::spawn().ok().unwrap();
        test_251::spawn().ok().unwrap();
        test_252::spawn().ok().unwrap();
        test_253::spawn().ok().unwrap();
        test_254::spawn().ok().unwrap();
        test_255::spawn().ok().unwrap();
        test_256::spawn().ok().unwrap();
        test_257::spawn().ok().unwrap();
        test_258::spawn().ok().unwrap();
        test_259::spawn().ok().unwrap();
        test_260::spawn().ok().unwrap();
        test_261::spawn().ok().unwrap();
        test_262::spawn().ok().unwrap();
        test_263::spawn().ok().unwrap();
        test_264::spawn().ok().unwrap();
        test_265::spawn().ok().unwrap();
        test_266::spawn().ok().unwrap();
        test_267::spawn().ok().unwrap();
        test_268::spawn().ok().unwrap();
        test_269::spawn().ok().unwrap();
        test_270::spawn().ok().unwrap();
        test_271::spawn().ok().unwrap();
        test_272::spawn().ok().unwrap();
        test_273::spawn().ok().unwrap();
        test_274::spawn().ok().unwrap();
        test_275::spawn().ok().unwrap();
        test_276::spawn().ok().unwrap();
        test_277::spawn().ok().unwrap();
        test_278::spawn().ok().unwrap();
        test_279::spawn().ok().unwrap();
        test_280::spawn().ok().unwrap();
        test_281::spawn().ok().unwrap();
        test_282::spawn().ok().unwrap();
        test_283::spawn().ok().unwrap();
        test_284::spawn().ok().unwrap();
        test_285::spawn().ok().unwrap();
        test_286::spawn().ok().unwrap();
        test_287::spawn().ok().unwrap();
        test_288::spawn().ok().unwrap();
        test_289::spawn().ok().unwrap();
        test_290::spawn().ok().unwrap();
        test_291::spawn().ok().unwrap();
        test_292::spawn().ok().unwrap();
        test_293::spawn().ok().unwrap();
        test_294::spawn().ok().unwrap();
        test_295::spawn().ok().unwrap();
        test_296::spawn().ok().unwrap();
        test_297::spawn().ok().unwrap();
        test_298::spawn().ok().unwrap();
        test_299::spawn().ok().unwrap();
        test_300::spawn().ok().unwrap();
        test_301::spawn().ok().unwrap();
        test_302::spawn().ok().unwrap();
        test_303::spawn().ok().unwrap();
        test_304::spawn().ok().unwrap();
        test_305::spawn().ok().unwrap();
        test_306::spawn().ok().unwrap();
        test_307::spawn().ok().unwrap();
        test_308::spawn().ok().unwrap();
        test_309::spawn().ok().unwrap();
        test_310::spawn().ok().unwrap();
        test_311::spawn().ok().unwrap();
        test_312::spawn().ok().unwrap();
        test_313::spawn().ok().unwrap();
        test_314::spawn().ok().unwrap();
        test_315::spawn().ok().unwrap();
        test_316::spawn().ok().unwrap();
        test_317::spawn().ok().unwrap();
        test_318::spawn().ok().unwrap();
        test_319::spawn().ok().unwrap();
        test_320::spawn().ok().unwrap();
        test_321::spawn().ok().unwrap();
        test_322::spawn().ok().unwrap();
        test_323::spawn().ok().unwrap();
        test_324::spawn().ok().unwrap();
        test_325::spawn().ok().unwrap();
        test_326::spawn().ok().unwrap();
        test_327::spawn().ok().unwrap();
        test_328::spawn().ok().unwrap();
        test_329::spawn().ok().unwrap();
        test_330::spawn().ok().unwrap();
        test_331::spawn().ok().unwrap();
        test_332::spawn().ok().unwrap();
        test_333::spawn().ok().unwrap();
        test_334::spawn().ok().unwrap();
        test_335::spawn().ok().unwrap();
        test_336::spawn().ok().unwrap();
        test_337::spawn().ok().unwrap();
        test_338::spawn().ok().unwrap();
        test_339::spawn().ok().unwrap();
        test_340::spawn().ok().unwrap();
        test_341::spawn().ok().unwrap();
        test_342::spawn().ok().unwrap();
        test_343::spawn().ok().unwrap();
        test_344::spawn().ok().unwrap();
        test_345::spawn().ok().unwrap();
        test_346::spawn().ok().unwrap();
        test_347::spawn().ok().unwrap();
        test_348::spawn().ok().unwrap();
        test_349::spawn().ok().unwrap();
        test_350::spawn().ok().unwrap();
        test_351::spawn().ok().unwrap();
        test_352::spawn().ok().unwrap();
        test_353::spawn().ok().unwrap();
        test_354::spawn().ok().unwrap();
        test_355::spawn().ok().unwrap();
        test_356::spawn().ok().unwrap();
        test_357::spawn().ok().unwrap();
        test_358::spawn().ok().unwrap();
        test_359::spawn().ok().unwrap();
        test_360::spawn().ok().unwrap();
        test_361::spawn().ok().unwrap();
        test_362::spawn().ok().unwrap();
        test_363::spawn().ok().unwrap();
        test_364::spawn().ok().unwrap();
        test_365::spawn().ok().unwrap();
        test_366::spawn().ok().unwrap();
        test_367::spawn().ok().unwrap();
        test_368::spawn().ok().unwrap();
        test_369::spawn().ok().unwrap();
        test_370::spawn().ok().unwrap();
        test_371::spawn().ok().unwrap();
        test_372::spawn().ok().unwrap();
        test_373::spawn().ok().unwrap();
        test_374::spawn().ok().unwrap();
        test_375::spawn().ok().unwrap();
        test_376::spawn().ok().unwrap();
        test_377::spawn().ok().unwrap();
        test_378::spawn().ok().unwrap();
        test_379::spawn().ok().unwrap();
        test_380::spawn().ok().unwrap();
        test_381::spawn().ok().unwrap();
        test_382::spawn().ok().unwrap();
        test_383::spawn().ok().unwrap();
        test_384::spawn().ok().unwrap();
        test_385::spawn().ok().unwrap();
        test_386::spawn().ok().unwrap();
        test_387::spawn().ok().unwrap();
        test_388::spawn().ok().unwrap();
        test_389::spawn().ok().unwrap();
        test_390::spawn().ok().unwrap();
        test_391::spawn().ok().unwrap();
        test_392::spawn().ok().unwrap();
        test_393::spawn().ok().unwrap();
        test_394::spawn().ok().unwrap();
        test_395::spawn().ok().unwrap();
        test_396::spawn().ok().unwrap();
        test_397::spawn().ok().unwrap();
        test_398::spawn().ok().unwrap();
        test_399::spawn().ok().unwrap();
        test_400::spawn().ok().unwrap();
        test_401::spawn().ok().unwrap();
        test_402::spawn().ok().unwrap();
        test_403::spawn().ok().unwrap();
        test_404::spawn().ok().unwrap();
        test_405::spawn().ok().unwrap();
        test_406::spawn().ok().unwrap();
        test_407::spawn().ok().unwrap();
        test_408::spawn().ok().unwrap();
        test_409::spawn().ok().unwrap();
        test_410::spawn().ok().unwrap();
        test_411::spawn().ok().unwrap();
        test_412::spawn().ok().unwrap();
        test_413::spawn().ok().unwrap();
        test_414::spawn().ok().unwrap();
        test_415::spawn().ok().unwrap();
        test_416::spawn().ok().unwrap();
        test_417::spawn().ok().unwrap();
        test_418::spawn().ok().unwrap();
        test_419::spawn().ok().unwrap();
        test_420::spawn().ok().unwrap();
        test_421::spawn().ok().unwrap();
        test_422::spawn().ok().unwrap();
        test_423::spawn().ok().unwrap();
        test_424::spawn().ok().unwrap();
        test_425::spawn().ok().unwrap();
        test_426::spawn().ok().unwrap();
        test_427::spawn().ok().unwrap();
        test_428::spawn().ok().unwrap();
        test_429::spawn().ok().unwrap();
        test_430::spawn().ok().unwrap();
        test_431::spawn().ok().unwrap();
        test_432::spawn().ok().unwrap();
        test_433::spawn().ok().unwrap();
        test_434::spawn().ok().unwrap();
        test_435::spawn().ok().unwrap();
        test_436::spawn().ok().unwrap();
        test_437::spawn().ok().unwrap();
        test_438::spawn().ok().unwrap();
        test_439::spawn().ok().unwrap();
        test_440::spawn().ok().unwrap();
        test_441::spawn().ok().unwrap();
        test_442::spawn().ok().unwrap();
        test_443::spawn().ok().unwrap();
        test_444::spawn().ok().unwrap();
        test_445::spawn().ok().unwrap();
        test_446::spawn().ok().unwrap();
        test_447::spawn().ok().unwrap();
        test_448::spawn().ok().unwrap();
        test_449::spawn().ok().unwrap();
        test_450::spawn().ok().unwrap();
        test_451::spawn().ok().unwrap();
        test_452::spawn().ok().unwrap();
        test_453::spawn().ok().unwrap();
        test_454::spawn().ok().unwrap();
        test_455::spawn().ok().unwrap();
        test_456::spawn().ok().unwrap();
        test_457::spawn().ok().unwrap();
        test_458::spawn().ok().unwrap();
        test_459::spawn().ok().unwrap();
        test_460::spawn().ok().unwrap();
        test_461::spawn().ok().unwrap();
        test_462::spawn().ok().unwrap();
        test_463::spawn().ok().unwrap();
        test_464::spawn().ok().unwrap();
        test_465::spawn().ok().unwrap();
        test_466::spawn().ok().unwrap();
        test_467::spawn().ok().unwrap();
        test_468::spawn().ok().unwrap();
        test_469::spawn().ok().unwrap();
        test_470::spawn().ok().unwrap();
        test_471::spawn().ok().unwrap();
        test_472::spawn().ok().unwrap();
        test_473::spawn().ok().unwrap();
        test_474::spawn().ok().unwrap();
        test_475::spawn().ok().unwrap();
        test_476::spawn().ok().unwrap();
        test_477::spawn().ok().unwrap();
        test_478::spawn().ok().unwrap();
        test_479::spawn().ok().unwrap();
        test_480::spawn().ok().unwrap();
        test_481::spawn().ok().unwrap();
        test_482::spawn().ok().unwrap();
        test_483::spawn().ok().unwrap();
        test_484::spawn().ok().unwrap();
        test_485::spawn().ok().unwrap();
        test_486::spawn().ok().unwrap();
        test_487::spawn().ok().unwrap();
        test_488::spawn().ok().unwrap();
        test_489::spawn().ok().unwrap();
        test_490::spawn().ok().unwrap();
        test_491::spawn().ok().unwrap();
        test_492::spawn().ok().unwrap();
        test_493::spawn().ok().unwrap();
        test_494::spawn().ok().unwrap();
        test_495::spawn().ok().unwrap();
        test_496::spawn().ok().unwrap();
        test_497::spawn().ok().unwrap();
        test_498::spawn().ok().unwrap();
        test_499::spawn().ok().unwrap();
        test_500::spawn().ok().unwrap();
        test_501::spawn().ok().unwrap();
        test_502::spawn().ok().unwrap();
        test_503::spawn().ok().unwrap();
        test_504::spawn().ok().unwrap();
        test_505::spawn().ok().unwrap();
        test_506::spawn().ok().unwrap();
        test_507::spawn().ok().unwrap();
        test_508::spawn().ok().unwrap();
        test_509::spawn().ok().unwrap();
        test_510::spawn().ok().unwrap();
        test_511::spawn().ok().unwrap();
        test_512::spawn().ok().unwrap();
        test_513::spawn().ok().unwrap();
        test_514::spawn().ok().unwrap();
        test_515::spawn().ok().unwrap();
        test_516::spawn().ok().unwrap();
        test_517::spawn().ok().unwrap();
        test_518::spawn().ok().unwrap();
        test_519::spawn().ok().unwrap();
        test_520::spawn().ok().unwrap();
        test_521::spawn().ok().unwrap();
        test_522::spawn().ok().unwrap();
        test_523::spawn().ok().unwrap();
        test_524::spawn().ok().unwrap();
        test_525::spawn().ok().unwrap();
        test_526::spawn().ok().unwrap();
        test_527::spawn().ok().unwrap();
        test_528::spawn().ok().unwrap();
        test_529::spawn().ok().unwrap();
        test_530::spawn().ok().unwrap();
        test_531::spawn().ok().unwrap();
        test_532::spawn().ok().unwrap();
        test_533::spawn().ok().unwrap();
        test_534::spawn().ok().unwrap();
        test_535::spawn().ok().unwrap();
        test_536::spawn().ok().unwrap();
        test_537::spawn().ok().unwrap();
        test_538::spawn().ok().unwrap();
        test_539::spawn().ok().unwrap();
        test_540::spawn().ok().unwrap();
        test_541::spawn().ok().unwrap();
        test_542::spawn().ok().unwrap();
        test_543::spawn().ok().unwrap();
        test_544::spawn().ok().unwrap();
        test_545::spawn().ok().unwrap();
        test_546::spawn().ok().unwrap();
        test_547::spawn().ok().unwrap();
        test_548::spawn().ok().unwrap();
        test_549::spawn().ok().unwrap();
        test_550::spawn().ok().unwrap();
        test_551::spawn().ok().unwrap();
        test_552::spawn().ok().unwrap();
        test_553::spawn().ok().unwrap();
        test_554::spawn().ok().unwrap();
        test_555::spawn().ok().unwrap();
        test_556::spawn().ok().unwrap();
        test_557::spawn().ok().unwrap();
        test_558::spawn().ok().unwrap();
        test_559::spawn().ok().unwrap();
        test_560::spawn().ok().unwrap();
        test_561::spawn().ok().unwrap();
        test_562::spawn().ok().unwrap();
        test_563::spawn().ok().unwrap();
        test_564::spawn().ok().unwrap();
        test_565::spawn().ok().unwrap();
        test_566::spawn().ok().unwrap();
        test_567::spawn().ok().unwrap();
        test_568::spawn().ok().unwrap();
        test_569::spawn().ok().unwrap();
        test_570::spawn().ok().unwrap();
        test_571::spawn().ok().unwrap();
        test_572::spawn().ok().unwrap();
        test_573::spawn().ok().unwrap();
        test_574::spawn().ok().unwrap();
        test_575::spawn().ok().unwrap();
        test_576::spawn().ok().unwrap();
        test_577::spawn().ok().unwrap();
        test_578::spawn().ok().unwrap();
        test_579::spawn().ok().unwrap();
        test_580::spawn().ok().unwrap();
        test_581::spawn().ok().unwrap();
        test_582::spawn().ok().unwrap();
        test_583::spawn().ok().unwrap();
        test_584::spawn().ok().unwrap();
        test_585::spawn().ok().unwrap();
        test_586::spawn().ok().unwrap();
        test_587::spawn().ok().unwrap();
        test_588::spawn().ok().unwrap();
        test_589::spawn().ok().unwrap();
        test_590::spawn().ok().unwrap();
        test_591::spawn().ok().unwrap();
        test_592::spawn().ok().unwrap();
        test_593::spawn().ok().unwrap();
        test_594::spawn().ok().unwrap();
        test_595::spawn().ok().unwrap();
        test_596::spawn().ok().unwrap();
        test_597::spawn().ok().unwrap();
        test_598::spawn().ok().unwrap();
        test_599::spawn().ok().unwrap();
        test_600::spawn().ok().unwrap();
        test_601::spawn().ok().unwrap();
        test_602::spawn().ok().unwrap();
        test_603::spawn().ok().unwrap();
        test_604::spawn().ok().unwrap();
        test_605::spawn().ok().unwrap();
        test_606::spawn().ok().unwrap();
        test_607::spawn().ok().unwrap();
        test_608::spawn().ok().unwrap();
        test_609::spawn().ok().unwrap();
        test_610::spawn().ok().unwrap();
        test_611::spawn().ok().unwrap();
        test_612::spawn().ok().unwrap();
        test_613::spawn().ok().unwrap();
        test_614::spawn().ok().unwrap();
        test_615::spawn().ok().unwrap();
        test_616::spawn().ok().unwrap();
        test_617::spawn().ok().unwrap();
        test_618::spawn().ok().unwrap();
        test_619::spawn().ok().unwrap();
        test_620::spawn().ok().unwrap();
        test_621::spawn().ok().unwrap();
        test_622::spawn().ok().unwrap();
        test_623::spawn().ok().unwrap();
        test_624::spawn().ok().unwrap();
        test_625::spawn().ok().unwrap();
        test_626::spawn().ok().unwrap();
        test_627::spawn().ok().unwrap();
        test_628::spawn().ok().unwrap();
        test_629::spawn().ok().unwrap();
        test_630::spawn().ok().unwrap();
        test_631::spawn().ok().unwrap();
        test_632::spawn().ok().unwrap();
        test_633::spawn().ok().unwrap();
        test_634::spawn().ok().unwrap();
        test_635::spawn().ok().unwrap();
        test_636::spawn().ok().unwrap();
        test_637::spawn().ok().unwrap();
        test_638::spawn().ok().unwrap();
        test_639::spawn().ok().unwrap();
        test_640::spawn().ok().unwrap();
        test_641::spawn().ok().unwrap();
        test_642::spawn().ok().unwrap();
        test_643::spawn().ok().unwrap();
        test_644::spawn().ok().unwrap();
        test_645::spawn().ok().unwrap();
        test_646::spawn().ok().unwrap();
        test_647::spawn().ok().unwrap();
        test_648::spawn().ok().unwrap();
        test_649::spawn().ok().unwrap();
        test_650::spawn().ok().unwrap();
        test_651::spawn().ok().unwrap();
        test_652::spawn().ok().unwrap();
        test_653::spawn().ok().unwrap();
        test_654::spawn().ok().unwrap();
        test_655::spawn().ok().unwrap();
        test_656::spawn().ok().unwrap();
        test_657::spawn().ok().unwrap();
        test_658::spawn().ok().unwrap();
        test_659::spawn().ok().unwrap();
        test_660::spawn().ok().unwrap();
        test_661::spawn().ok().unwrap();
        test_662::spawn().ok().unwrap();
        test_663::spawn().ok().unwrap();
        test_664::spawn().ok().unwrap();
        test_665::spawn().ok().unwrap();
        test_666::spawn().ok().unwrap();
        test_667::spawn().ok().unwrap();
        test_668::spawn().ok().unwrap();
        test_669::spawn().ok().unwrap();
        test_670::spawn().ok().unwrap();
        test_671::spawn().ok().unwrap();
        test_672::spawn().ok().unwrap();
        test_673::spawn().ok().unwrap();
        test_674::spawn().ok().unwrap();
        test_675::spawn().ok().unwrap();
        test_676::spawn().ok().unwrap();
        test_677::spawn().ok().unwrap();
        test_678::spawn().ok().unwrap();
        test_679::spawn().ok().unwrap();
        test_680::spawn().ok().unwrap();
        test_681::spawn().ok().unwrap();
        test_682::spawn().ok().unwrap();
        test_683::spawn().ok().unwrap();
        test_684::spawn().ok().unwrap();
        test_685::spawn().ok().unwrap();
        test_686::spawn().ok().unwrap();
        test_687::spawn().ok().unwrap();
        test_688::spawn().ok().unwrap();
        test_689::spawn().ok().unwrap();
        test_690::spawn().ok().unwrap();
        test_691::spawn().ok().unwrap();
        test_692::spawn().ok().unwrap();
        test_693::spawn().ok().unwrap();
        test_694::spawn().ok().unwrap();
        test_695::spawn().ok().unwrap();
        test_696::spawn().ok().unwrap();
        test_697::spawn().ok().unwrap();
        test_698::spawn().ok().unwrap();
        test_699::spawn().ok().unwrap();
        test_700::spawn().ok().unwrap();
        test_701::spawn().ok().unwrap();
        test_702::spawn().ok().unwrap();
        test_703::spawn().ok().unwrap();
        test_704::spawn().ok().unwrap();
        test_705::spawn().ok().unwrap();
        test_706::spawn().ok().unwrap();
        test_707::spawn().ok().unwrap();
        test_708::spawn().ok().unwrap();
        test_709::spawn().ok().unwrap();
        test_710::spawn().ok().unwrap();
        test_711::spawn().ok().unwrap();
        test_712::spawn().ok().unwrap();
        test_713::spawn().ok().unwrap();
        test_714::spawn().ok().unwrap();
        test_715::spawn().ok().unwrap();
        test_716::spawn().ok().unwrap();
        test_717::spawn().ok().unwrap();
        test_718::spawn().ok().unwrap();
        test_719::spawn().ok().unwrap();
        test_720::spawn().ok().unwrap();
        test_721::spawn().ok().unwrap();
        test_722::spawn().ok().unwrap();
        test_723::spawn().ok().unwrap();
        test_724::spawn().ok().unwrap();
        test_725::spawn().ok().unwrap();
        test_726::spawn().ok().unwrap();
        test_727::spawn().ok().unwrap();
        test_728::spawn().ok().unwrap();
        test_729::spawn().ok().unwrap();
        test_730::spawn().ok().unwrap();
        test_731::spawn().ok().unwrap();
        test_732::spawn().ok().unwrap();
        test_733::spawn().ok().unwrap();
        test_734::spawn().ok().unwrap();
        test_735::spawn().ok().unwrap();
        test_736::spawn().ok().unwrap();
        test_737::spawn().ok().unwrap();
        test_738::spawn().ok().unwrap();
        test_739::spawn().ok().unwrap();
        test_740::spawn().ok().unwrap();
        test_741::spawn().ok().unwrap();
        test_742::spawn().ok().unwrap();
        test_743::spawn().ok().unwrap();
        test_744::spawn().ok().unwrap();
        test_745::spawn().ok().unwrap();
        test_746::spawn().ok().unwrap();
        test_747::spawn().ok().unwrap();
        test_748::spawn().ok().unwrap();
        test_749::spawn().ok().unwrap();
        test_750::spawn().ok().unwrap();
        test_751::spawn().ok().unwrap();
        test_752::spawn().ok().unwrap();
        test_753::spawn().ok().unwrap();
        test_754::spawn().ok().unwrap();
        test_755::spawn().ok().unwrap();
        test_756::spawn().ok().unwrap();
        test_757::spawn().ok().unwrap();
        test_758::spawn().ok().unwrap();
        test_759::spawn().ok().unwrap();
        test_760::spawn().ok().unwrap();
        test_761::spawn().ok().unwrap();
        test_762::spawn().ok().unwrap();
        test_763::spawn().ok().unwrap();
        test_764::spawn().ok().unwrap();
        test_765::spawn().ok().unwrap();
        test_766::spawn().ok().unwrap();
        test_767::spawn().ok().unwrap();
        test_768::spawn().ok().unwrap();
        test_clock::spawn().ok().unwrap();

        (
            Shared {
                display: pkg.display,
            },
            Local {
                red_led: pkg.red_led,
                neopixels: pkg.neopixels,
            },
        )
    }

    #[idle(local = [red_led])]
    fn idle(cx: idle::Context) -> ! {
        loop {
            rtic::export::wfi();
            cx.local.red_led.toggle().unwrap();
        }
    }

    #[task(priority = 1, local=[neopixels])]
    async fn test_neopixels(cx: test_neopixels::Context) {
        neopixels_task(cx.local.neopixels, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_1(cx: test_1::Context) {
        test_task(cx.shared.display, 0, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_2(cx: test_2::Context) {
        test_task(cx.shared.display, 1, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_3(cx: test_3::Context) {
        test_task(cx.shared.display, 2, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_4(cx: test_4::Context) {
        test_task(cx.shared.display, 3, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_5(cx: test_5::Context) {
        test_task(cx.shared.display, 4, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_6(cx: test_6::Context) {
        test_task(cx.shared.display, 5, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_7(cx: test_7::Context) {
        test_task(cx.shared.display, 6, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_8(cx: test_8::Context) {
        test_task(cx.shared.display, 7, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_9(cx: test_9::Context) {
        test_task(cx.shared.display, 8, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_10(cx: test_10::Context) {
        test_task(cx.shared.display, 9, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_11(cx: test_11::Context) {
        test_task(cx.shared.display, 10, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_12(cx: test_12::Context) {
        test_task(cx.shared.display, 11, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_13(cx: test_13::Context) {
        test_task(cx.shared.display, 12, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_14(cx: test_14::Context) {
        test_task(cx.shared.display, 13, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_15(cx: test_15::Context) {
        test_task(cx.shared.display, 14, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_16(cx: test_16::Context) {
        test_task(cx.shared.display, 15, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_17(cx: test_17::Context) {
        test_task(cx.shared.display, 16, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_18(cx: test_18::Context) {
        test_task(cx.shared.display, 17, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_19(cx: test_19::Context) {
        test_task(cx.shared.display, 18, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_20(cx: test_20::Context) {
        test_task(cx.shared.display, 19, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_21(cx: test_21::Context) {
        test_task(cx.shared.display, 20, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_22(cx: test_22::Context) {
        test_task(cx.shared.display, 21, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_23(cx: test_23::Context) {
        test_task(cx.shared.display, 22, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_24(cx: test_24::Context) {
        test_task(cx.shared.display, 23, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_25(cx: test_25::Context) {
        test_task(cx.shared.display, 24, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_26(cx: test_26::Context) {
        test_task(cx.shared.display, 25, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_27(cx: test_27::Context) {
        test_task(cx.shared.display, 26, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_28(cx: test_28::Context) {
        test_task(cx.shared.display, 27, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_29(cx: test_29::Context) {
        test_task(cx.shared.display, 28, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_30(cx: test_30::Context) {
        test_task(cx.shared.display, 29, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_31(cx: test_31::Context) {
        test_task(cx.shared.display, 30, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_32(cx: test_32::Context) {
        test_task(cx.shared.display, 31, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_33(cx: test_33::Context) {
        test_task(cx.shared.display, 32, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_34(cx: test_34::Context) {
        test_task(cx.shared.display, 33, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_35(cx: test_35::Context) {
        test_task(cx.shared.display, 34, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_36(cx: test_36::Context) {
        test_task(cx.shared.display, 35, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_37(cx: test_37::Context) {
        test_task(cx.shared.display, 36, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_38(cx: test_38::Context) {
        test_task(cx.shared.display, 37, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_39(cx: test_39::Context) {
        test_task(cx.shared.display, 38, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_40(cx: test_40::Context) {
        test_task(cx.shared.display, 39, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_41(cx: test_41::Context) {
        test_task(cx.shared.display, 40, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_42(cx: test_42::Context) {
        test_task(cx.shared.display, 41, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_43(cx: test_43::Context) {
        test_task(cx.shared.display, 42, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_44(cx: test_44::Context) {
        test_task(cx.shared.display, 43, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_45(cx: test_45::Context) {
        test_task(cx.shared.display, 44, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_46(cx: test_46::Context) {
        test_task(cx.shared.display, 45, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_47(cx: test_47::Context) {
        test_task(cx.shared.display, 46, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_48(cx: test_48::Context) {
        test_task(cx.shared.display, 47, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_49(cx: test_49::Context) {
        test_task(cx.shared.display, 48, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_50(cx: test_50::Context) {
        test_task(cx.shared.display, 49, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_51(cx: test_51::Context) {
        test_task(cx.shared.display, 50, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_52(cx: test_52::Context) {
        test_task(cx.shared.display, 51, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_53(cx: test_53::Context) {
        test_task(cx.shared.display, 52, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_54(cx: test_54::Context) {
        test_task(cx.shared.display, 53, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_55(cx: test_55::Context) {
        test_task(cx.shared.display, 54, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_56(cx: test_56::Context) {
        test_task(cx.shared.display, 55, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_57(cx: test_57::Context) {
        test_task(cx.shared.display, 56, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_58(cx: test_58::Context) {
        test_task(cx.shared.display, 57, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_59(cx: test_59::Context) {
        test_task(cx.shared.display, 58, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_60(cx: test_60::Context) {
        test_task(cx.shared.display, 59, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_61(cx: test_61::Context) {
        test_task(cx.shared.display, 60, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_62(cx: test_62::Context) {
        test_task(cx.shared.display, 61, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_63(cx: test_63::Context) {
        test_task(cx.shared.display, 62, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_64(cx: test_64::Context) {
        test_task(cx.shared.display, 63, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_65(cx: test_65::Context) {
        test_task(cx.shared.display, 64, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_66(cx: test_66::Context) {
        test_task(cx.shared.display, 65, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_67(cx: test_67::Context) {
        test_task(cx.shared.display, 66, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_68(cx: test_68::Context) {
        test_task(cx.shared.display, 67, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_69(cx: test_69::Context) {
        test_task(cx.shared.display, 68, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_70(cx: test_70::Context) {
        test_task(cx.shared.display, 69, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_71(cx: test_71::Context) {
        test_task(cx.shared.display, 70, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_72(cx: test_72::Context) {
        test_task(cx.shared.display, 71, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_73(cx: test_73::Context) {
        test_task(cx.shared.display, 72, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_74(cx: test_74::Context) {
        test_task(cx.shared.display, 73, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_75(cx: test_75::Context) {
        test_task(cx.shared.display, 74, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_76(cx: test_76::Context) {
        test_task(cx.shared.display, 75, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_77(cx: test_77::Context) {
        test_task(cx.shared.display, 76, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_78(cx: test_78::Context) {
        test_task(cx.shared.display, 77, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_79(cx: test_79::Context) {
        test_task(cx.shared.display, 78, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_80(cx: test_80::Context) {
        test_task(cx.shared.display, 79, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_81(cx: test_81::Context) {
        test_task(cx.shared.display, 80, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_82(cx: test_82::Context) {
        test_task(cx.shared.display, 81, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_83(cx: test_83::Context) {
        test_task(cx.shared.display, 82, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_84(cx: test_84::Context) {
        test_task(cx.shared.display, 83, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_85(cx: test_85::Context) {
        test_task(cx.shared.display, 84, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_86(cx: test_86::Context) {
        test_task(cx.shared.display, 85, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_87(cx: test_87::Context) {
        test_task(cx.shared.display, 86, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_88(cx: test_88::Context) {
        test_task(cx.shared.display, 87, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_89(cx: test_89::Context) {
        test_task(cx.shared.display, 88, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_90(cx: test_90::Context) {
        test_task(cx.shared.display, 89, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_91(cx: test_91::Context) {
        test_task(cx.shared.display, 90, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_92(cx: test_92::Context) {
        test_task(cx.shared.display, 91, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_93(cx: test_93::Context) {
        test_task(cx.shared.display, 92, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_94(cx: test_94::Context) {
        test_task(cx.shared.display, 93, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_95(cx: test_95::Context) {
        test_task(cx.shared.display, 94, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_96(cx: test_96::Context) {
        test_task(cx.shared.display, 95, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_97(cx: test_97::Context) {
        test_task(cx.shared.display, 96, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_98(cx: test_98::Context) {
        test_task(cx.shared.display, 97, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_99(cx: test_99::Context) {
        test_task(cx.shared.display, 98, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_100(cx: test_100::Context) {
        test_task(cx.shared.display, 99, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_101(cx: test_101::Context) {
        test_task(cx.shared.display, 100, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_102(cx: test_102::Context) {
        test_task(cx.shared.display, 101, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_103(cx: test_103::Context) {
        test_task(cx.shared.display, 102, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_104(cx: test_104::Context) {
        test_task(cx.shared.display, 103, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_105(cx: test_105::Context) {
        test_task(cx.shared.display, 104, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_106(cx: test_106::Context) {
        test_task(cx.shared.display, 105, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_107(cx: test_107::Context) {
        test_task(cx.shared.display, 106, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_108(cx: test_108::Context) {
        test_task(cx.shared.display, 107, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_109(cx: test_109::Context) {
        test_task(cx.shared.display, 108, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_110(cx: test_110::Context) {
        test_task(cx.shared.display, 109, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_111(cx: test_111::Context) {
        test_task(cx.shared.display, 110, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_112(cx: test_112::Context) {
        test_task(cx.shared.display, 111, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_113(cx: test_113::Context) {
        test_task(cx.shared.display, 112, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_114(cx: test_114::Context) {
        test_task(cx.shared.display, 113, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_115(cx: test_115::Context) {
        test_task(cx.shared.display, 114, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_116(cx: test_116::Context) {
        test_task(cx.shared.display, 115, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_117(cx: test_117::Context) {
        test_task(cx.shared.display, 116, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_118(cx: test_118::Context) {
        test_task(cx.shared.display, 117, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_119(cx: test_119::Context) {
        test_task(cx.shared.display, 118, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_120(cx: test_120::Context) {
        test_task(cx.shared.display, 119, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_121(cx: test_121::Context) {
        test_task(cx.shared.display, 120, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_122(cx: test_122::Context) {
        test_task(cx.shared.display, 121, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_123(cx: test_123::Context) {
        test_task(cx.shared.display, 122, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_124(cx: test_124::Context) {
        test_task(cx.shared.display, 123, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_125(cx: test_125::Context) {
        test_task(cx.shared.display, 124, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_126(cx: test_126::Context) {
        test_task(cx.shared.display, 125, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_127(cx: test_127::Context) {
        test_task(cx.shared.display, 126, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_128(cx: test_128::Context) {
        test_task(cx.shared.display, 127, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_129(cx: test_129::Context) {
        test_task(cx.shared.display, 128, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_130(cx: test_130::Context) {
        test_task(cx.shared.display, 129, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_131(cx: test_131::Context) {
        test_task(cx.shared.display, 130, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_132(cx: test_132::Context) {
        test_task(cx.shared.display, 131, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_133(cx: test_133::Context) {
        test_task(cx.shared.display, 132, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_134(cx: test_134::Context) {
        test_task(cx.shared.display, 133, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_135(cx: test_135::Context) {
        test_task(cx.shared.display, 134, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_136(cx: test_136::Context) {
        test_task(cx.shared.display, 135, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_137(cx: test_137::Context) {
        test_task(cx.shared.display, 136, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_138(cx: test_138::Context) {
        test_task(cx.shared.display, 137, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_139(cx: test_139::Context) {
        test_task(cx.shared.display, 138, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_140(cx: test_140::Context) {
        test_task(cx.shared.display, 139, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_141(cx: test_141::Context) {
        test_task(cx.shared.display, 140, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_142(cx: test_142::Context) {
        test_task(cx.shared.display, 141, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_143(cx: test_143::Context) {
        test_task(cx.shared.display, 142, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_144(cx: test_144::Context) {
        test_task(cx.shared.display, 143, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_145(cx: test_145::Context) {
        test_task(cx.shared.display, 144, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_146(cx: test_146::Context) {
        test_task(cx.shared.display, 145, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_147(cx: test_147::Context) {
        test_task(cx.shared.display, 146, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_148(cx: test_148::Context) {
        test_task(cx.shared.display, 147, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_149(cx: test_149::Context) {
        test_task(cx.shared.display, 148, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_150(cx: test_150::Context) {
        test_task(cx.shared.display, 149, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_151(cx: test_151::Context) {
        test_task(cx.shared.display, 150, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_152(cx: test_152::Context) {
        test_task(cx.shared.display, 151, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_153(cx: test_153::Context) {
        test_task(cx.shared.display, 152, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_154(cx: test_154::Context) {
        test_task(cx.shared.display, 153, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_155(cx: test_155::Context) {
        test_task(cx.shared.display, 154, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_156(cx: test_156::Context) {
        test_task(cx.shared.display, 155, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_157(cx: test_157::Context) {
        test_task(cx.shared.display, 156, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_158(cx: test_158::Context) {
        test_task(cx.shared.display, 157, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_159(cx: test_159::Context) {
        test_task(cx.shared.display, 158, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_160(cx: test_160::Context) {
        test_task(cx.shared.display, 159, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_161(cx: test_161::Context) {
        test_task(cx.shared.display, 160, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_162(cx: test_162::Context) {
        test_task(cx.shared.display, 161, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_163(cx: test_163::Context) {
        test_task(cx.shared.display, 162, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_164(cx: test_164::Context) {
        test_task(cx.shared.display, 163, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_165(cx: test_165::Context) {
        test_task(cx.shared.display, 164, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_166(cx: test_166::Context) {
        test_task(cx.shared.display, 165, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_167(cx: test_167::Context) {
        test_task(cx.shared.display, 166, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_168(cx: test_168::Context) {
        test_task(cx.shared.display, 167, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_169(cx: test_169::Context) {
        test_task(cx.shared.display, 168, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_170(cx: test_170::Context) {
        test_task(cx.shared.display, 169, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_171(cx: test_171::Context) {
        test_task(cx.shared.display, 170, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_172(cx: test_172::Context) {
        test_task(cx.shared.display, 171, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_173(cx: test_173::Context) {
        test_task(cx.shared.display, 172, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_174(cx: test_174::Context) {
        test_task(cx.shared.display, 173, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_175(cx: test_175::Context) {
        test_task(cx.shared.display, 174, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_176(cx: test_176::Context) {
        test_task(cx.shared.display, 175, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_177(cx: test_177::Context) {
        test_task(cx.shared.display, 176, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_178(cx: test_178::Context) {
        test_task(cx.shared.display, 177, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_179(cx: test_179::Context) {
        test_task(cx.shared.display, 178, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_180(cx: test_180::Context) {
        test_task(cx.shared.display, 179, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_181(cx: test_181::Context) {
        test_task(cx.shared.display, 180, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_182(cx: test_182::Context) {
        test_task(cx.shared.display, 181, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_183(cx: test_183::Context) {
        test_task(cx.shared.display, 182, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_184(cx: test_184::Context) {
        test_task(cx.shared.display, 183, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_185(cx: test_185::Context) {
        test_task(cx.shared.display, 184, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_186(cx: test_186::Context) {
        test_task(cx.shared.display, 185, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_187(cx: test_187::Context) {
        test_task(cx.shared.display, 186, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_188(cx: test_188::Context) {
        test_task(cx.shared.display, 187, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_189(cx: test_189::Context) {
        test_task(cx.shared.display, 188, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_190(cx: test_190::Context) {
        test_task(cx.shared.display, 189, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_191(cx: test_191::Context) {
        test_task(cx.shared.display, 190, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_192(cx: test_192::Context) {
        test_task(cx.shared.display, 191, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_193(cx: test_193::Context) {
        test_task(cx.shared.display, 192, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_194(cx: test_194::Context) {
        test_task(cx.shared.display, 193, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_195(cx: test_195::Context) {
        test_task(cx.shared.display, 194, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_196(cx: test_196::Context) {
        test_task(cx.shared.display, 195, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_197(cx: test_197::Context) {
        test_task(cx.shared.display, 196, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_198(cx: test_198::Context) {
        test_task(cx.shared.display, 197, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_199(cx: test_199::Context) {
        test_task(cx.shared.display, 198, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_200(cx: test_200::Context) {
        test_task(cx.shared.display, 199, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_201(cx: test_201::Context) {
        test_task(cx.shared.display, 200, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_202(cx: test_202::Context) {
        test_task(cx.shared.display, 201, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_203(cx: test_203::Context) {
        test_task(cx.shared.display, 202, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_204(cx: test_204::Context) {
        test_task(cx.shared.display, 203, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_205(cx: test_205::Context) {
        test_task(cx.shared.display, 204, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_206(cx: test_206::Context) {
        test_task(cx.shared.display, 205, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_207(cx: test_207::Context) {
        test_task(cx.shared.display, 206, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_208(cx: test_208::Context) {
        test_task(cx.shared.display, 207, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_209(cx: test_209::Context) {
        test_task(cx.shared.display, 208, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_210(cx: test_210::Context) {
        test_task(cx.shared.display, 209, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_211(cx: test_211::Context) {
        test_task(cx.shared.display, 210, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_212(cx: test_212::Context) {
        test_task(cx.shared.display, 211, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_213(cx: test_213::Context) {
        test_task(cx.shared.display, 212, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_214(cx: test_214::Context) {
        test_task(cx.shared.display, 213, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_215(cx: test_215::Context) {
        test_task(cx.shared.display, 214, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_216(cx: test_216::Context) {
        test_task(cx.shared.display, 215, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_217(cx: test_217::Context) {
        test_task(cx.shared.display, 216, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_218(cx: test_218::Context) {
        test_task(cx.shared.display, 217, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_219(cx: test_219::Context) {
        test_task(cx.shared.display, 218, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_220(cx: test_220::Context) {
        test_task(cx.shared.display, 219, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_221(cx: test_221::Context) {
        test_task(cx.shared.display, 220, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_222(cx: test_222::Context) {
        test_task(cx.shared.display, 221, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_223(cx: test_223::Context) {
        test_task(cx.shared.display, 222, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_224(cx: test_224::Context) {
        test_task(cx.shared.display, 223, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_225(cx: test_225::Context) {
        test_task(cx.shared.display, 224, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_226(cx: test_226::Context) {
        test_task(cx.shared.display, 225, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_227(cx: test_227::Context) {
        test_task(cx.shared.display, 226, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_228(cx: test_228::Context) {
        test_task(cx.shared.display, 227, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_229(cx: test_229::Context) {
        test_task(cx.shared.display, 228, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_230(cx: test_230::Context) {
        test_task(cx.shared.display, 229, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_231(cx: test_231::Context) {
        test_task(cx.shared.display, 230, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_232(cx: test_232::Context) {
        test_task(cx.shared.display, 231, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_233(cx: test_233::Context) {
        test_task(cx.shared.display, 232, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_234(cx: test_234::Context) {
        test_task(cx.shared.display, 233, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_235(cx: test_235::Context) {
        test_task(cx.shared.display, 234, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_236(cx: test_236::Context) {
        test_task(cx.shared.display, 235, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_237(cx: test_237::Context) {
        test_task(cx.shared.display, 236, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_238(cx: test_238::Context) {
        test_task(cx.shared.display, 237, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_239(cx: test_239::Context) {
        test_task(cx.shared.display, 238, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_240(cx: test_240::Context) {
        test_task(cx.shared.display, 239, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_241(cx: test_241::Context) {
        test_task(cx.shared.display, 240, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_242(cx: test_242::Context) {
        test_task(cx.shared.display, 241, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_243(cx: test_243::Context) {
        test_task(cx.shared.display, 242, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_244(cx: test_244::Context) {
        test_task(cx.shared.display, 243, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_245(cx: test_245::Context) {
        test_task(cx.shared.display, 244, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_246(cx: test_246::Context) {
        test_task(cx.shared.display, 245, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_247(cx: test_247::Context) {
        test_task(cx.shared.display, 246, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_248(cx: test_248::Context) {
        test_task(cx.shared.display, 247, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_249(cx: test_249::Context) {
        test_task(cx.shared.display, 248, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_250(cx: test_250::Context) {
        test_task(cx.shared.display, 249, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_251(cx: test_251::Context) {
        test_task(cx.shared.display, 250, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_252(cx: test_252::Context) {
        test_task(cx.shared.display, 251, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_253(cx: test_253::Context) {
        test_task(cx.shared.display, 252, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_254(cx: test_254::Context) {
        test_task(cx.shared.display, 253, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_255(cx: test_255::Context) {
        test_task(cx.shared.display, 254, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_256(cx: test_256::Context) {
        test_task(cx.shared.display, 255, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_257(cx: test_257::Context) {
        test_task(cx.shared.display, 256, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_258(cx: test_258::Context) {
        test_task(cx.shared.display, 257, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_259(cx: test_259::Context) {
        test_task(cx.shared.display, 258, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_260(cx: test_260::Context) {
        test_task(cx.shared.display, 259, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_261(cx: test_261::Context) {
        test_task(cx.shared.display, 260, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_262(cx: test_262::Context) {
        test_task(cx.shared.display, 261, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_263(cx: test_263::Context) {
        test_task(cx.shared.display, 262, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_264(cx: test_264::Context) {
        test_task(cx.shared.display, 263, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_265(cx: test_265::Context) {
        test_task(cx.shared.display, 264, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_266(cx: test_266::Context) {
        test_task(cx.shared.display, 265, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_267(cx: test_267::Context) {
        test_task(cx.shared.display, 266, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_268(cx: test_268::Context) {
        test_task(cx.shared.display, 267, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_269(cx: test_269::Context) {
        test_task(cx.shared.display, 268, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_270(cx: test_270::Context) {
        test_task(cx.shared.display, 269, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_271(cx: test_271::Context) {
        test_task(cx.shared.display, 270, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_272(cx: test_272::Context) {
        test_task(cx.shared.display, 271, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_273(cx: test_273::Context) {
        test_task(cx.shared.display, 272, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_274(cx: test_274::Context) {
        test_task(cx.shared.display, 273, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_275(cx: test_275::Context) {
        test_task(cx.shared.display, 274, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_276(cx: test_276::Context) {
        test_task(cx.shared.display, 275, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_277(cx: test_277::Context) {
        test_task(cx.shared.display, 276, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_278(cx: test_278::Context) {
        test_task(cx.shared.display, 277, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_279(cx: test_279::Context) {
        test_task(cx.shared.display, 278, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_280(cx: test_280::Context) {
        test_task(cx.shared.display, 279, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_281(cx: test_281::Context) {
        test_task(cx.shared.display, 280, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_282(cx: test_282::Context) {
        test_task(cx.shared.display, 281, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_283(cx: test_283::Context) {
        test_task(cx.shared.display, 282, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_284(cx: test_284::Context) {
        test_task(cx.shared.display, 283, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_285(cx: test_285::Context) {
        test_task(cx.shared.display, 284, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_286(cx: test_286::Context) {
        test_task(cx.shared.display, 285, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_287(cx: test_287::Context) {
        test_task(cx.shared.display, 286, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_288(cx: test_288::Context) {
        test_task(cx.shared.display, 287, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_289(cx: test_289::Context) {
        test_task(cx.shared.display, 288, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_290(cx: test_290::Context) {
        test_task(cx.shared.display, 289, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_291(cx: test_291::Context) {
        test_task(cx.shared.display, 290, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_292(cx: test_292::Context) {
        test_task(cx.shared.display, 291, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_293(cx: test_293::Context) {
        test_task(cx.shared.display, 292, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_294(cx: test_294::Context) {
        test_task(cx.shared.display, 293, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_295(cx: test_295::Context) {
        test_task(cx.shared.display, 294, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_296(cx: test_296::Context) {
        test_task(cx.shared.display, 295, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_297(cx: test_297::Context) {
        test_task(cx.shared.display, 296, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_298(cx: test_298::Context) {
        test_task(cx.shared.display, 297, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_299(cx: test_299::Context) {
        test_task(cx.shared.display, 298, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_300(cx: test_300::Context) {
        test_task(cx.shared.display, 299, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_301(cx: test_301::Context) {
        test_task(cx.shared.display, 300, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_302(cx: test_302::Context) {
        test_task(cx.shared.display, 301, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_303(cx: test_303::Context) {
        test_task(cx.shared.display, 302, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_304(cx: test_304::Context) {
        test_task(cx.shared.display, 303, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_305(cx: test_305::Context) {
        test_task(cx.shared.display, 304, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_306(cx: test_306::Context) {
        test_task(cx.shared.display, 305, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_307(cx: test_307::Context) {
        test_task(cx.shared.display, 306, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_308(cx: test_308::Context) {
        test_task(cx.shared.display, 307, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_309(cx: test_309::Context) {
        test_task(cx.shared.display, 308, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_310(cx: test_310::Context) {
        test_task(cx.shared.display, 309, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_311(cx: test_311::Context) {
        test_task(cx.shared.display, 310, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_312(cx: test_312::Context) {
        test_task(cx.shared.display, 311, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_313(cx: test_313::Context) {
        test_task(cx.shared.display, 312, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_314(cx: test_314::Context) {
        test_task(cx.shared.display, 313, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_315(cx: test_315::Context) {
        test_task(cx.shared.display, 314, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_316(cx: test_316::Context) {
        test_task(cx.shared.display, 315, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_317(cx: test_317::Context) {
        test_task(cx.shared.display, 316, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_318(cx: test_318::Context) {
        test_task(cx.shared.display, 317, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_319(cx: test_319::Context) {
        test_task(cx.shared.display, 318, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_320(cx: test_320::Context) {
        test_task(cx.shared.display, 319, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_321(cx: test_321::Context) {
        test_task(cx.shared.display, 320, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_322(cx: test_322::Context) {
        test_task(cx.shared.display, 321, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_323(cx: test_323::Context) {
        test_task(cx.shared.display, 322, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_324(cx: test_324::Context) {
        test_task(cx.shared.display, 323, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_325(cx: test_325::Context) {
        test_task(cx.shared.display, 324, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_326(cx: test_326::Context) {
        test_task(cx.shared.display, 325, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_327(cx: test_327::Context) {
        test_task(cx.shared.display, 326, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_328(cx: test_328::Context) {
        test_task(cx.shared.display, 327, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_329(cx: test_329::Context) {
        test_task(cx.shared.display, 328, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_330(cx: test_330::Context) {
        test_task(cx.shared.display, 329, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_331(cx: test_331::Context) {
        test_task(cx.shared.display, 330, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_332(cx: test_332::Context) {
        test_task(cx.shared.display, 331, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_333(cx: test_333::Context) {
        test_task(cx.shared.display, 332, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_334(cx: test_334::Context) {
        test_task(cx.shared.display, 333, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_335(cx: test_335::Context) {
        test_task(cx.shared.display, 334, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_336(cx: test_336::Context) {
        test_task(cx.shared.display, 335, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_337(cx: test_337::Context) {
        test_task(cx.shared.display, 336, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_338(cx: test_338::Context) {
        test_task(cx.shared.display, 337, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_339(cx: test_339::Context) {
        test_task(cx.shared.display, 338, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_340(cx: test_340::Context) {
        test_task(cx.shared.display, 339, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_341(cx: test_341::Context) {
        test_task(cx.shared.display, 340, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_342(cx: test_342::Context) {
        test_task(cx.shared.display, 341, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_343(cx: test_343::Context) {
        test_task(cx.shared.display, 342, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_344(cx: test_344::Context) {
        test_task(cx.shared.display, 343, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_345(cx: test_345::Context) {
        test_task(cx.shared.display, 344, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_346(cx: test_346::Context) {
        test_task(cx.shared.display, 345, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_347(cx: test_347::Context) {
        test_task(cx.shared.display, 346, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_348(cx: test_348::Context) {
        test_task(cx.shared.display, 347, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_349(cx: test_349::Context) {
        test_task(cx.shared.display, 348, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_350(cx: test_350::Context) {
        test_task(cx.shared.display, 349, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_351(cx: test_351::Context) {
        test_task(cx.shared.display, 350, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_352(cx: test_352::Context) {
        test_task(cx.shared.display, 351, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_353(cx: test_353::Context) {
        test_task(cx.shared.display, 352, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_354(cx: test_354::Context) {
        test_task(cx.shared.display, 353, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_355(cx: test_355::Context) {
        test_task(cx.shared.display, 354, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_356(cx: test_356::Context) {
        test_task(cx.shared.display, 355, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_357(cx: test_357::Context) {
        test_task(cx.shared.display, 356, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_358(cx: test_358::Context) {
        test_task(cx.shared.display, 357, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_359(cx: test_359::Context) {
        test_task(cx.shared.display, 358, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_360(cx: test_360::Context) {
        test_task(cx.shared.display, 359, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_361(cx: test_361::Context) {
        test_task(cx.shared.display, 360, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_362(cx: test_362::Context) {
        test_task(cx.shared.display, 361, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_363(cx: test_363::Context) {
        test_task(cx.shared.display, 362, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_364(cx: test_364::Context) {
        test_task(cx.shared.display, 363, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_365(cx: test_365::Context) {
        test_task(cx.shared.display, 364, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_366(cx: test_366::Context) {
        test_task(cx.shared.display, 365, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_367(cx: test_367::Context) {
        test_task(cx.shared.display, 366, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_368(cx: test_368::Context) {
        test_task(cx.shared.display, 367, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_369(cx: test_369::Context) {
        test_task(cx.shared.display, 368, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_370(cx: test_370::Context) {
        test_task(cx.shared.display, 369, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_371(cx: test_371::Context) {
        test_task(cx.shared.display, 370, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_372(cx: test_372::Context) {
        test_task(cx.shared.display, 371, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_373(cx: test_373::Context) {
        test_task(cx.shared.display, 372, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_374(cx: test_374::Context) {
        test_task(cx.shared.display, 373, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_375(cx: test_375::Context) {
        test_task(cx.shared.display, 374, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_376(cx: test_376::Context) {
        test_task(cx.shared.display, 375, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_377(cx: test_377::Context) {
        test_task(cx.shared.display, 376, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_378(cx: test_378::Context) {
        test_task(cx.shared.display, 377, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_379(cx: test_379::Context) {
        test_task(cx.shared.display, 378, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_380(cx: test_380::Context) {
        test_task(cx.shared.display, 379, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_381(cx: test_381::Context) {
        test_task(cx.shared.display, 380, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_382(cx: test_382::Context) {
        test_task(cx.shared.display, 381, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_383(cx: test_383::Context) {
        test_task(cx.shared.display, 382, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_384(cx: test_384::Context) {
        test_task(cx.shared.display, 383, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_385(cx: test_385::Context) {
        test_task(cx.shared.display, 384, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_386(cx: test_386::Context) {
        test_task(cx.shared.display, 385, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_387(cx: test_387::Context) {
        test_task(cx.shared.display, 386, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_388(cx: test_388::Context) {
        test_task(cx.shared.display, 387, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_389(cx: test_389::Context) {
        test_task(cx.shared.display, 388, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_390(cx: test_390::Context) {
        test_task(cx.shared.display, 389, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_391(cx: test_391::Context) {
        test_task(cx.shared.display, 390, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_392(cx: test_392::Context) {
        test_task(cx.shared.display, 391, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_393(cx: test_393::Context) {
        test_task(cx.shared.display, 392, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_394(cx: test_394::Context) {
        test_task(cx.shared.display, 393, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_395(cx: test_395::Context) {
        test_task(cx.shared.display, 394, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_396(cx: test_396::Context) {
        test_task(cx.shared.display, 395, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_397(cx: test_397::Context) {
        test_task(cx.shared.display, 396, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_398(cx: test_398::Context) {
        test_task(cx.shared.display, 397, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_399(cx: test_399::Context) {
        test_task(cx.shared.display, 398, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_400(cx: test_400::Context) {
        test_task(cx.shared.display, 399, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_401(cx: test_401::Context) {
        test_task(cx.shared.display, 400, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_402(cx: test_402::Context) {
        test_task(cx.shared.display, 401, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_403(cx: test_403::Context) {
        test_task(cx.shared.display, 402, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_404(cx: test_404::Context) {
        test_task(cx.shared.display, 403, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_405(cx: test_405::Context) {
        test_task(cx.shared.display, 404, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_406(cx: test_406::Context) {
        test_task(cx.shared.display, 405, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_407(cx: test_407::Context) {
        test_task(cx.shared.display, 406, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_408(cx: test_408::Context) {
        test_task(cx.shared.display, 407, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_409(cx: test_409::Context) {
        test_task(cx.shared.display, 408, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_410(cx: test_410::Context) {
        test_task(cx.shared.display, 409, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_411(cx: test_411::Context) {
        test_task(cx.shared.display, 410, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_412(cx: test_412::Context) {
        test_task(cx.shared.display, 411, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_413(cx: test_413::Context) {
        test_task(cx.shared.display, 412, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_414(cx: test_414::Context) {
        test_task(cx.shared.display, 413, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_415(cx: test_415::Context) {
        test_task(cx.shared.display, 414, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_416(cx: test_416::Context) {
        test_task(cx.shared.display, 415, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_417(cx: test_417::Context) {
        test_task(cx.shared.display, 416, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_418(cx: test_418::Context) {
        test_task(cx.shared.display, 417, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_419(cx: test_419::Context) {
        test_task(cx.shared.display, 418, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_420(cx: test_420::Context) {
        test_task(cx.shared.display, 419, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_421(cx: test_421::Context) {
        test_task(cx.shared.display, 420, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_422(cx: test_422::Context) {
        test_task(cx.shared.display, 421, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_423(cx: test_423::Context) {
        test_task(cx.shared.display, 422, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_424(cx: test_424::Context) {
        test_task(cx.shared.display, 423, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_425(cx: test_425::Context) {
        test_task(cx.shared.display, 424, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_426(cx: test_426::Context) {
        test_task(cx.shared.display, 425, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_427(cx: test_427::Context) {
        test_task(cx.shared.display, 426, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_428(cx: test_428::Context) {
        test_task(cx.shared.display, 427, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_429(cx: test_429::Context) {
        test_task(cx.shared.display, 428, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_430(cx: test_430::Context) {
        test_task(cx.shared.display, 429, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_431(cx: test_431::Context) {
        test_task(cx.shared.display, 430, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_432(cx: test_432::Context) {
        test_task(cx.shared.display, 431, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_433(cx: test_433::Context) {
        test_task(cx.shared.display, 432, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_434(cx: test_434::Context) {
        test_task(cx.shared.display, 433, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_435(cx: test_435::Context) {
        test_task(cx.shared.display, 434, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_436(cx: test_436::Context) {
        test_task(cx.shared.display, 435, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_437(cx: test_437::Context) {
        test_task(cx.shared.display, 436, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_438(cx: test_438::Context) {
        test_task(cx.shared.display, 437, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_439(cx: test_439::Context) {
        test_task(cx.shared.display, 438, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_440(cx: test_440::Context) {
        test_task(cx.shared.display, 439, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_441(cx: test_441::Context) {
        test_task(cx.shared.display, 440, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_442(cx: test_442::Context) {
        test_task(cx.shared.display, 441, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_443(cx: test_443::Context) {
        test_task(cx.shared.display, 442, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_444(cx: test_444::Context) {
        test_task(cx.shared.display, 443, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_445(cx: test_445::Context) {
        test_task(cx.shared.display, 444, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_446(cx: test_446::Context) {
        test_task(cx.shared.display, 445, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_447(cx: test_447::Context) {
        test_task(cx.shared.display, 446, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_448(cx: test_448::Context) {
        test_task(cx.shared.display, 447, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_449(cx: test_449::Context) {
        test_task(cx.shared.display, 448, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_450(cx: test_450::Context) {
        test_task(cx.shared.display, 449, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_451(cx: test_451::Context) {
        test_task(cx.shared.display, 450, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_452(cx: test_452::Context) {
        test_task(cx.shared.display, 451, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_453(cx: test_453::Context) {
        test_task(cx.shared.display, 452, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_454(cx: test_454::Context) {
        test_task(cx.shared.display, 453, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_455(cx: test_455::Context) {
        test_task(cx.shared.display, 454, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_456(cx: test_456::Context) {
        test_task(cx.shared.display, 455, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_457(cx: test_457::Context) {
        test_task(cx.shared.display, 456, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_458(cx: test_458::Context) {
        test_task(cx.shared.display, 457, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_459(cx: test_459::Context) {
        test_task(cx.shared.display, 458, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_460(cx: test_460::Context) {
        test_task(cx.shared.display, 459, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_461(cx: test_461::Context) {
        test_task(cx.shared.display, 460, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_462(cx: test_462::Context) {
        test_task(cx.shared.display, 461, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_463(cx: test_463::Context) {
        test_task(cx.shared.display, 462, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_464(cx: test_464::Context) {
        test_task(cx.shared.display, 463, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_465(cx: test_465::Context) {
        test_task(cx.shared.display, 464, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_466(cx: test_466::Context) {
        test_task(cx.shared.display, 465, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_467(cx: test_467::Context) {
        test_task(cx.shared.display, 466, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_468(cx: test_468::Context) {
        test_task(cx.shared.display, 467, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_469(cx: test_469::Context) {
        test_task(cx.shared.display, 468, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_470(cx: test_470::Context) {
        test_task(cx.shared.display, 469, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_471(cx: test_471::Context) {
        test_task(cx.shared.display, 470, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_472(cx: test_472::Context) {
        test_task(cx.shared.display, 471, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_473(cx: test_473::Context) {
        test_task(cx.shared.display, 472, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_474(cx: test_474::Context) {
        test_task(cx.shared.display, 473, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_475(cx: test_475::Context) {
        test_task(cx.shared.display, 474, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_476(cx: test_476::Context) {
        test_task(cx.shared.display, 475, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_477(cx: test_477::Context) {
        test_task(cx.shared.display, 476, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_478(cx: test_478::Context) {
        test_task(cx.shared.display, 477, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_479(cx: test_479::Context) {
        test_task(cx.shared.display, 478, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_480(cx: test_480::Context) {
        test_task(cx.shared.display, 479, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_481(cx: test_481::Context) {
        test_task(cx.shared.display, 480, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_482(cx: test_482::Context) {
        test_task(cx.shared.display, 481, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_483(cx: test_483::Context) {
        test_task(cx.shared.display, 482, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_484(cx: test_484::Context) {
        test_task(cx.shared.display, 483, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_485(cx: test_485::Context) {
        test_task(cx.shared.display, 484, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_486(cx: test_486::Context) {
        test_task(cx.shared.display, 485, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_487(cx: test_487::Context) {
        test_task(cx.shared.display, 486, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_488(cx: test_488::Context) {
        test_task(cx.shared.display, 487, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_489(cx: test_489::Context) {
        test_task(cx.shared.display, 488, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_490(cx: test_490::Context) {
        test_task(cx.shared.display, 489, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_491(cx: test_491::Context) {
        test_task(cx.shared.display, 490, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_492(cx: test_492::Context) {
        test_task(cx.shared.display, 491, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_493(cx: test_493::Context) {
        test_task(cx.shared.display, 492, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_494(cx: test_494::Context) {
        test_task(cx.shared.display, 493, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_495(cx: test_495::Context) {
        test_task(cx.shared.display, 494, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_496(cx: test_496::Context) {
        test_task(cx.shared.display, 495, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_497(cx: test_497::Context) {
        test_task(cx.shared.display, 496, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_498(cx: test_498::Context) {
        test_task(cx.shared.display, 497, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_499(cx: test_499::Context) {
        test_task(cx.shared.display, 498, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_500(cx: test_500::Context) {
        test_task(cx.shared.display, 499, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_501(cx: test_501::Context) {
        test_task(cx.shared.display, 500, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_502(cx: test_502::Context) {
        test_task(cx.shared.display, 501, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_503(cx: test_503::Context) {
        test_task(cx.shared.display, 502, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_504(cx: test_504::Context) {
        test_task(cx.shared.display, 503, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_505(cx: test_505::Context) {
        test_task(cx.shared.display, 504, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_506(cx: test_506::Context) {
        test_task(cx.shared.display, 505, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_507(cx: test_507::Context) {
        test_task(cx.shared.display, 506, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_508(cx: test_508::Context) {
        test_task(cx.shared.display, 507, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_509(cx: test_509::Context) {
        test_task(cx.shared.display, 508, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_510(cx: test_510::Context) {
        test_task(cx.shared.display, 509, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_511(cx: test_511::Context) {
        test_task(cx.shared.display, 510, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_512(cx: test_512::Context) {
        test_task(cx.shared.display, 511, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_513(cx: test_513::Context) {
        test_task(cx.shared.display, 512, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_514(cx: test_514::Context) {
        test_task(cx.shared.display, 513, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_515(cx: test_515::Context) {
        test_task(cx.shared.display, 514, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_516(cx: test_516::Context) {
        test_task(cx.shared.display, 515, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_517(cx: test_517::Context) {
        test_task(cx.shared.display, 516, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_518(cx: test_518::Context) {
        test_task(cx.shared.display, 517, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_519(cx: test_519::Context) {
        test_task(cx.shared.display, 518, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_520(cx: test_520::Context) {
        test_task(cx.shared.display, 519, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_521(cx: test_521::Context) {
        test_task(cx.shared.display, 520, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_522(cx: test_522::Context) {
        test_task(cx.shared.display, 521, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_523(cx: test_523::Context) {
        test_task(cx.shared.display, 522, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_524(cx: test_524::Context) {
        test_task(cx.shared.display, 523, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_525(cx: test_525::Context) {
        test_task(cx.shared.display, 524, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_526(cx: test_526::Context) {
        test_task(cx.shared.display, 525, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_527(cx: test_527::Context) {
        test_task(cx.shared.display, 526, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_528(cx: test_528::Context) {
        test_task(cx.shared.display, 527, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_529(cx: test_529::Context) {
        test_task(cx.shared.display, 528, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_530(cx: test_530::Context) {
        test_task(cx.shared.display, 529, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_531(cx: test_531::Context) {
        test_task(cx.shared.display, 530, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_532(cx: test_532::Context) {
        test_task(cx.shared.display, 531, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_533(cx: test_533::Context) {
        test_task(cx.shared.display, 532, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_534(cx: test_534::Context) {
        test_task(cx.shared.display, 533, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_535(cx: test_535::Context) {
        test_task(cx.shared.display, 534, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_536(cx: test_536::Context) {
        test_task(cx.shared.display, 535, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_537(cx: test_537::Context) {
        test_task(cx.shared.display, 536, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_538(cx: test_538::Context) {
        test_task(cx.shared.display, 537, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_539(cx: test_539::Context) {
        test_task(cx.shared.display, 538, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_540(cx: test_540::Context) {
        test_task(cx.shared.display, 539, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_541(cx: test_541::Context) {
        test_task(cx.shared.display, 540, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_542(cx: test_542::Context) {
        test_task(cx.shared.display, 541, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_543(cx: test_543::Context) {
        test_task(cx.shared.display, 542, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_544(cx: test_544::Context) {
        test_task(cx.shared.display, 543, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_545(cx: test_545::Context) {
        test_task(cx.shared.display, 544, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_546(cx: test_546::Context) {
        test_task(cx.shared.display, 545, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_547(cx: test_547::Context) {
        test_task(cx.shared.display, 546, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_548(cx: test_548::Context) {
        test_task(cx.shared.display, 547, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_549(cx: test_549::Context) {
        test_task(cx.shared.display, 548, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_550(cx: test_550::Context) {
        test_task(cx.shared.display, 549, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_551(cx: test_551::Context) {
        test_task(cx.shared.display, 550, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_552(cx: test_552::Context) {
        test_task(cx.shared.display, 551, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_553(cx: test_553::Context) {
        test_task(cx.shared.display, 552, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_554(cx: test_554::Context) {
        test_task(cx.shared.display, 553, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_555(cx: test_555::Context) {
        test_task(cx.shared.display, 554, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_556(cx: test_556::Context) {
        test_task(cx.shared.display, 555, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_557(cx: test_557::Context) {
        test_task(cx.shared.display, 556, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_558(cx: test_558::Context) {
        test_task(cx.shared.display, 557, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_559(cx: test_559::Context) {
        test_task(cx.shared.display, 558, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_560(cx: test_560::Context) {
        test_task(cx.shared.display, 559, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_561(cx: test_561::Context) {
        test_task(cx.shared.display, 560, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_562(cx: test_562::Context) {
        test_task(cx.shared.display, 561, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_563(cx: test_563::Context) {
        test_task(cx.shared.display, 562, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_564(cx: test_564::Context) {
        test_task(cx.shared.display, 563, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_565(cx: test_565::Context) {
        test_task(cx.shared.display, 564, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_566(cx: test_566::Context) {
        test_task(cx.shared.display, 565, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_567(cx: test_567::Context) {
        test_task(cx.shared.display, 566, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_568(cx: test_568::Context) {
        test_task(cx.shared.display, 567, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_569(cx: test_569::Context) {
        test_task(cx.shared.display, 568, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_570(cx: test_570::Context) {
        test_task(cx.shared.display, 569, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_571(cx: test_571::Context) {
        test_task(cx.shared.display, 570, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_572(cx: test_572::Context) {
        test_task(cx.shared.display, 571, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_573(cx: test_573::Context) {
        test_task(cx.shared.display, 572, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_574(cx: test_574::Context) {
        test_task(cx.shared.display, 573, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_575(cx: test_575::Context) {
        test_task(cx.shared.display, 574, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_576(cx: test_576::Context) {
        test_task(cx.shared.display, 575, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_577(cx: test_577::Context) {
        test_task(cx.shared.display, 576, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_578(cx: test_578::Context) {
        test_task(cx.shared.display, 577, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_579(cx: test_579::Context) {
        test_task(cx.shared.display, 578, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_580(cx: test_580::Context) {
        test_task(cx.shared.display, 579, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_581(cx: test_581::Context) {
        test_task(cx.shared.display, 580, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_582(cx: test_582::Context) {
        test_task(cx.shared.display, 581, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_583(cx: test_583::Context) {
        test_task(cx.shared.display, 582, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_584(cx: test_584::Context) {
        test_task(cx.shared.display, 583, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_585(cx: test_585::Context) {
        test_task(cx.shared.display, 584, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_586(cx: test_586::Context) {
        test_task(cx.shared.display, 585, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_587(cx: test_587::Context) {
        test_task(cx.shared.display, 586, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_588(cx: test_588::Context) {
        test_task(cx.shared.display, 587, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_589(cx: test_589::Context) {
        test_task(cx.shared.display, 588, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_590(cx: test_590::Context) {
        test_task(cx.shared.display, 589, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_591(cx: test_591::Context) {
        test_task(cx.shared.display, 590, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_592(cx: test_592::Context) {
        test_task(cx.shared.display, 591, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_593(cx: test_593::Context) {
        test_task(cx.shared.display, 592, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_594(cx: test_594::Context) {
        test_task(cx.shared.display, 593, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_595(cx: test_595::Context) {
        test_task(cx.shared.display, 594, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_596(cx: test_596::Context) {
        test_task(cx.shared.display, 595, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_597(cx: test_597::Context) {
        test_task(cx.shared.display, 596, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_598(cx: test_598::Context) {
        test_task(cx.shared.display, 597, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_599(cx: test_599::Context) {
        test_task(cx.shared.display, 598, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_600(cx: test_600::Context) {
        test_task(cx.shared.display, 599, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_601(cx: test_601::Context) {
        test_task(cx.shared.display, 600, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_602(cx: test_602::Context) {
        test_task(cx.shared.display, 601, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_603(cx: test_603::Context) {
        test_task(cx.shared.display, 602, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_604(cx: test_604::Context) {
        test_task(cx.shared.display, 603, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_605(cx: test_605::Context) {
        test_task(cx.shared.display, 604, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_606(cx: test_606::Context) {
        test_task(cx.shared.display, 605, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_607(cx: test_607::Context) {
        test_task(cx.shared.display, 606, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_608(cx: test_608::Context) {
        test_task(cx.shared.display, 607, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_609(cx: test_609::Context) {
        test_task(cx.shared.display, 608, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_610(cx: test_610::Context) {
        test_task(cx.shared.display, 609, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_611(cx: test_611::Context) {
        test_task(cx.shared.display, 610, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_612(cx: test_612::Context) {
        test_task(cx.shared.display, 611, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_613(cx: test_613::Context) {
        test_task(cx.shared.display, 612, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_614(cx: test_614::Context) {
        test_task(cx.shared.display, 613, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_615(cx: test_615::Context) {
        test_task(cx.shared.display, 614, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_616(cx: test_616::Context) {
        test_task(cx.shared.display, 615, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_617(cx: test_617::Context) {
        test_task(cx.shared.display, 616, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_618(cx: test_618::Context) {
        test_task(cx.shared.display, 617, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_619(cx: test_619::Context) {
        test_task(cx.shared.display, 618, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_620(cx: test_620::Context) {
        test_task(cx.shared.display, 619, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_621(cx: test_621::Context) {
        test_task(cx.shared.display, 620, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_622(cx: test_622::Context) {
        test_task(cx.shared.display, 621, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_623(cx: test_623::Context) {
        test_task(cx.shared.display, 622, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_624(cx: test_624::Context) {
        test_task(cx.shared.display, 623, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_625(cx: test_625::Context) {
        test_task(cx.shared.display, 624, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_626(cx: test_626::Context) {
        test_task(cx.shared.display, 625, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_627(cx: test_627::Context) {
        test_task(cx.shared.display, 626, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_628(cx: test_628::Context) {
        test_task(cx.shared.display, 627, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_629(cx: test_629::Context) {
        test_task(cx.shared.display, 628, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_630(cx: test_630::Context) {
        test_task(cx.shared.display, 629, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_631(cx: test_631::Context) {
        test_task(cx.shared.display, 630, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_632(cx: test_632::Context) {
        test_task(cx.shared.display, 631, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_633(cx: test_633::Context) {
        test_task(cx.shared.display, 632, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_634(cx: test_634::Context) {
        test_task(cx.shared.display, 633, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_635(cx: test_635::Context) {
        test_task(cx.shared.display, 634, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_636(cx: test_636::Context) {
        test_task(cx.shared.display, 635, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_637(cx: test_637::Context) {
        test_task(cx.shared.display, 636, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_638(cx: test_638::Context) {
        test_task(cx.shared.display, 637, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_639(cx: test_639::Context) {
        test_task(cx.shared.display, 638, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_640(cx: test_640::Context) {
        test_task(cx.shared.display, 639, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_641(cx: test_641::Context) {
        test_task(cx.shared.display, 640, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_642(cx: test_642::Context) {
        test_task(cx.shared.display, 641, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_643(cx: test_643::Context) {
        test_task(cx.shared.display, 642, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_644(cx: test_644::Context) {
        test_task(cx.shared.display, 643, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_645(cx: test_645::Context) {
        test_task(cx.shared.display, 644, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_646(cx: test_646::Context) {
        test_task(cx.shared.display, 645, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_647(cx: test_647::Context) {
        test_task(cx.shared.display, 646, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_648(cx: test_648::Context) {
        test_task(cx.shared.display, 647, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_649(cx: test_649::Context) {
        test_task(cx.shared.display, 648, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_650(cx: test_650::Context) {
        test_task(cx.shared.display, 649, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_651(cx: test_651::Context) {
        test_task(cx.shared.display, 650, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_652(cx: test_652::Context) {
        test_task(cx.shared.display, 651, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_653(cx: test_653::Context) {
        test_task(cx.shared.display, 652, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_654(cx: test_654::Context) {
        test_task(cx.shared.display, 653, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_655(cx: test_655::Context) {
        test_task(cx.shared.display, 654, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_656(cx: test_656::Context) {
        test_task(cx.shared.display, 655, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_657(cx: test_657::Context) {
        test_task(cx.shared.display, 656, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_658(cx: test_658::Context) {
        test_task(cx.shared.display, 657, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_659(cx: test_659::Context) {
        test_task(cx.shared.display, 658, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_660(cx: test_660::Context) {
        test_task(cx.shared.display, 659, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_661(cx: test_661::Context) {
        test_task(cx.shared.display, 660, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_662(cx: test_662::Context) {
        test_task(cx.shared.display, 661, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_663(cx: test_663::Context) {
        test_task(cx.shared.display, 662, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_664(cx: test_664::Context) {
        test_task(cx.shared.display, 663, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_665(cx: test_665::Context) {
        test_task(cx.shared.display, 664, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_666(cx: test_666::Context) {
        test_task(cx.shared.display, 665, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_667(cx: test_667::Context) {
        test_task(cx.shared.display, 666, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_668(cx: test_668::Context) {
        test_task(cx.shared.display, 667, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_669(cx: test_669::Context) {
        test_task(cx.shared.display, 668, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_670(cx: test_670::Context) {
        test_task(cx.shared.display, 669, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_671(cx: test_671::Context) {
        test_task(cx.shared.display, 670, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_672(cx: test_672::Context) {
        test_task(cx.shared.display, 671, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_673(cx: test_673::Context) {
        test_task(cx.shared.display, 672, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_674(cx: test_674::Context) {
        test_task(cx.shared.display, 673, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_675(cx: test_675::Context) {
        test_task(cx.shared.display, 674, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_676(cx: test_676::Context) {
        test_task(cx.shared.display, 675, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_677(cx: test_677::Context) {
        test_task(cx.shared.display, 676, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_678(cx: test_678::Context) {
        test_task(cx.shared.display, 677, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_679(cx: test_679::Context) {
        test_task(cx.shared.display, 678, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_680(cx: test_680::Context) {
        test_task(cx.shared.display, 679, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_681(cx: test_681::Context) {
        test_task(cx.shared.display, 680, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_682(cx: test_682::Context) {
        test_task(cx.shared.display, 681, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_683(cx: test_683::Context) {
        test_task(cx.shared.display, 682, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_684(cx: test_684::Context) {
        test_task(cx.shared.display, 683, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_685(cx: test_685::Context) {
        test_task(cx.shared.display, 684, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_686(cx: test_686::Context) {
        test_task(cx.shared.display, 685, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_687(cx: test_687::Context) {
        test_task(cx.shared.display, 686, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_688(cx: test_688::Context) {
        test_task(cx.shared.display, 687, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_689(cx: test_689::Context) {
        test_task(cx.shared.display, 688, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_690(cx: test_690::Context) {
        test_task(cx.shared.display, 689, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_691(cx: test_691::Context) {
        test_task(cx.shared.display, 690, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_692(cx: test_692::Context) {
        test_task(cx.shared.display, 691, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_693(cx: test_693::Context) {
        test_task(cx.shared.display, 692, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_694(cx: test_694::Context) {
        test_task(cx.shared.display, 693, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_695(cx: test_695::Context) {
        test_task(cx.shared.display, 694, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_696(cx: test_696::Context) {
        test_task(cx.shared.display, 695, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_697(cx: test_697::Context) {
        test_task(cx.shared.display, 696, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_698(cx: test_698::Context) {
        test_task(cx.shared.display, 697, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_699(cx: test_699::Context) {
        test_task(cx.shared.display, 698, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_700(cx: test_700::Context) {
        test_task(cx.shared.display, 699, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_701(cx: test_701::Context) {
        test_task(cx.shared.display, 700, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_702(cx: test_702::Context) {
        test_task(cx.shared.display, 701, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_703(cx: test_703::Context) {
        test_task(cx.shared.display, 702, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_704(cx: test_704::Context) {
        test_task(cx.shared.display, 703, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_705(cx: test_705::Context) {
        test_task(cx.shared.display, 704, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_706(cx: test_706::Context) {
        test_task(cx.shared.display, 705, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_707(cx: test_707::Context) {
        test_task(cx.shared.display, 706, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_708(cx: test_708::Context) {
        test_task(cx.shared.display, 707, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_709(cx: test_709::Context) {
        test_task(cx.shared.display, 708, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_710(cx: test_710::Context) {
        test_task(cx.shared.display, 709, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_711(cx: test_711::Context) {
        test_task(cx.shared.display, 710, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_712(cx: test_712::Context) {
        test_task(cx.shared.display, 711, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_713(cx: test_713::Context) {
        test_task(cx.shared.display, 712, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_714(cx: test_714::Context) {
        test_task(cx.shared.display, 713, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_715(cx: test_715::Context) {
        test_task(cx.shared.display, 714, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_716(cx: test_716::Context) {
        test_task(cx.shared.display, 715, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_717(cx: test_717::Context) {
        test_task(cx.shared.display, 716, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_718(cx: test_718::Context) {
        test_task(cx.shared.display, 717, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_719(cx: test_719::Context) {
        test_task(cx.shared.display, 718, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_720(cx: test_720::Context) {
        test_task(cx.shared.display, 719, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_721(cx: test_721::Context) {
        test_task(cx.shared.display, 720, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_722(cx: test_722::Context) {
        test_task(cx.shared.display, 721, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_723(cx: test_723::Context) {
        test_task(cx.shared.display, 722, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_724(cx: test_724::Context) {
        test_task(cx.shared.display, 723, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_725(cx: test_725::Context) {
        test_task(cx.shared.display, 724, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_726(cx: test_726::Context) {
        test_task(cx.shared.display, 725, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_727(cx: test_727::Context) {
        test_task(cx.shared.display, 726, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_728(cx: test_728::Context) {
        test_task(cx.shared.display, 727, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_729(cx: test_729::Context) {
        test_task(cx.shared.display, 728, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_730(cx: test_730::Context) {
        test_task(cx.shared.display, 729, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_731(cx: test_731::Context) {
        test_task(cx.shared.display, 730, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_732(cx: test_732::Context) {
        test_task(cx.shared.display, 731, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_733(cx: test_733::Context) {
        test_task(cx.shared.display, 732, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_734(cx: test_734::Context) {
        test_task(cx.shared.display, 733, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_735(cx: test_735::Context) {
        test_task(cx.shared.display, 734, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_736(cx: test_736::Context) {
        test_task(cx.shared.display, 735, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_737(cx: test_737::Context) {
        test_task(cx.shared.display, 736, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_738(cx: test_738::Context) {
        test_task(cx.shared.display, 737, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_739(cx: test_739::Context) {
        test_task(cx.shared.display, 738, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_740(cx: test_740::Context) {
        test_task(cx.shared.display, 739, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_741(cx: test_741::Context) {
        test_task(cx.shared.display, 740, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_742(cx: test_742::Context) {
        test_task(cx.shared.display, 741, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_743(cx: test_743::Context) {
        test_task(cx.shared.display, 742, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_744(cx: test_744::Context) {
        test_task(cx.shared.display, 743, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_745(cx: test_745::Context) {
        test_task(cx.shared.display, 744, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_746(cx: test_746::Context) {
        test_task(cx.shared.display, 745, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_747(cx: test_747::Context) {
        test_task(cx.shared.display, 746, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_748(cx: test_748::Context) {
        test_task(cx.shared.display, 747, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_749(cx: test_749::Context) {
        test_task(cx.shared.display, 748, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_750(cx: test_750::Context) {
        test_task(cx.shared.display, 749, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_751(cx: test_751::Context) {
        test_task(cx.shared.display, 750, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_752(cx: test_752::Context) {
        test_task(cx.shared.display, 751, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_753(cx: test_753::Context) {
        test_task(cx.shared.display, 752, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_754(cx: test_754::Context) {
        test_task(cx.shared.display, 753, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_755(cx: test_755::Context) {
        test_task(cx.shared.display, 754, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_756(cx: test_756::Context) {
        test_task(cx.shared.display, 755, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_757(cx: test_757::Context) {
        test_task(cx.shared.display, 756, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_758(cx: test_758::Context) {
        test_task(cx.shared.display, 757, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_759(cx: test_759::Context) {
        test_task(cx.shared.display, 758, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_760(cx: test_760::Context) {
        test_task(cx.shared.display, 759, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_761(cx: test_761::Context) {
        test_task(cx.shared.display, 760, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_762(cx: test_762::Context) {
        test_task(cx.shared.display, 761, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_763(cx: test_763::Context) {
        test_task(cx.shared.display, 762, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_764(cx: test_764::Context) {
        test_task(cx.shared.display, 763, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_765(cx: test_765::Context) {
        test_task(cx.shared.display, 764, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_766(cx: test_766::Context) {
        test_task(cx.shared.display, 765, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_767(cx: test_767::Context) {
        test_task(cx.shared.display, 766, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_768(cx: test_768::Context) {
        test_task(cx.shared.display, 767, BASE_PERIOD_MS).await
    }

    #[task(priority = 1, shared=[display])]
    async fn test_clock(cx: test_clock::Context) {
        async_stress::clock_task(cx.shared.display).await
    }
}

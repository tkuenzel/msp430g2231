#[cfg(feature = "rt")]
global_asm!(
    "
                DH_TRAMPOLINE:
                    br #DEFAULT_HANDLER
                "
);
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak PORT1\nPORT1 = DH_TRAMPOLINE\n.weak PORT2\nPORT2 = DH_TRAMPOLINE\n.weak USI\nUSI = DH_TRAMPOLINE\n.weak ADC10\nADC10 = DH_TRAMPOLINE\n.weak TIMERA1\nTIMERA1 = DH_TRAMPOLINE\n.weak TIMERA0\nTIMERA0 = DH_TRAMPOLINE\n.weak WDT\nWDT = DH_TRAMPOLINE\n.weak NMI\nNMI = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT1();
    fn PORT2();
    fn USI();
    fn ADC10();
    fn TIMERA1();
    fn TIMERA0();
    fn WDT();
    fn NMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Vector; 15] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORT1 },
    Vector { _handler: PORT2 },
    Vector { _handler: USI },
    Vector { _handler: ADC10 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIMERA1 },
    Vector { _handler: TIMERA0 },
    Vector { _handler: WDT },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: NMI },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "2 - 0xFFE4 Port 1"]
    PORT1 = 2,
    #[doc = "3 - 0xFFE6 Port 2"]
    PORT2 = 3,
    #[doc = "4 - 0xFFE8 USI"]
    USI = 4,
    #[doc = "5 - 0xFFEA ADC10"]
    ADC10 = 5,
    #[doc = "8 - 0xFFF0 Timer A CC1, TA"]
    TIMERA1 = 8,
    #[doc = "9 - 0xFFF2 Timer A CC0"]
    TIMERA0 = 9,
    #[doc = "10 - 0xFFF4 Watchdog Timer"]
    WDT = 10,
    #[doc = "14 - 0xFFFC Non-maskable"]
    NMI = 14,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            2 => Ok(Interrupt::PORT1),
            3 => Ok(Interrupt::PORT2),
            4 => Ok(Interrupt::USI),
            5 => Ok(Interrupt::ADC10),
            8 => Ok(Interrupt::TIMERA1),
            9 => Ok(Interrupt::TIMERA0),
            10 => Ok(Interrupt::WDT),
            14 => Ok(Interrupt::NMI),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }

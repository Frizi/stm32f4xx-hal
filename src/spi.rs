use core::ops::Deref;
use core::ptr;

use crate::gpio::{Const, NoPin, SetAlternate};
use embedded_hal::spi;
pub use embedded_hal::spi::{Mode, Phase, Polarity};

#[allow(unused)]
#[cfg(feature = "gpiod")]
use crate::gpio::gpiod;
#[allow(unused)]
#[cfg(feature = "gpioe")]
use crate::gpio::gpioe;
#[allow(unused)]
#[cfg(feature = "gpiof")]
use crate::gpio::gpiof;
#[allow(unused)]
#[cfg(feature = "gpiog")]
use crate::gpio::gpiog;
#[allow(unused)]
use crate::gpio::gpioh;
#[allow(unused)]
#[cfg(feature = "gpioi")]
use crate::gpio::gpioi;
use crate::gpio::{gpioa, gpiob, gpioc};

use crate::pac::{spi1, RCC, SPI1, SPI2};
use crate::rcc;

#[cfg(feature = "spi3")]
use crate::pac::SPI3;

#[cfg(feature = "spi4")]
use crate::pac::SPI4;

#[cfg(feature = "spi5")]
use crate::pac::SPI5;

#[cfg(feature = "spi6")]
use crate::pac::SPI6;

use crate::rcc::Clocks;
use crate::time::Hertz;

/// SPI error
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
}

pub trait Pins<SPI> {}
pub trait PinSck<SPI> {
    type A;
}
pub trait PinMiso<SPI> {
    type A;
}
pub trait PinMosi<SPI> {
    type A;
}

impl<SPI, SCK, MISO, MOSI> Pins<SPI> for (SCK, MISO, MOSI)
where
    SCK: PinSck<SPI>,
    MISO: PinMiso<SPI>,
    MOSI: PinMosi<SPI>,
{
}

/// A filler type for when the SCK pin is unnecessary
pub type NoSck = NoPin;
/// A filler type for when the Miso pin is unnecessary
pub type NoMiso = NoPin;
/// A filler type for when the Mosi pin is unnecessary
pub type NoMosi = NoPin;

impl<SPI> PinSck<SPI> for NoPin
where
    SPI: Instance,
{
    type A = Const<0>;
}
impl<SPI> PinMiso<SPI> for NoPin
where
    SPI: Instance,
{
    type A = Const<0>;
}
impl<SPI> PinMosi<SPI> for NoPin
where
    SPI: Instance,
{
    type A = Const<0>;
}

macro_rules! pins {
    ($($SPIX:ty: SCK: [$($gpio1:ident::$PX1:ident<$A1:literal>),*]
                MISO: [$($gpio2:ident::$PX2:ident<$A2:literal>),*]
                MOSI: [$($gpio3:ident::$PX3:ident<$A3:literal>),*])+) => {
        $(
            $(
                impl<MODE> PinSck<$SPIX> for $gpio1::$PX1<MODE> {
                    type A = Const<$A1>;
                }
            )*
            $(
                impl<MODE> PinMiso<$SPIX> for $gpio2::$PX2<MODE> {
                    type A = Const<$A2>;
                }
            )*
            $(
                impl<MODE> PinMosi<$SPIX> for $gpio3::$PX3<MODE> {
                    type A = Const<$A3>;
                }
            )*
        )+
    }
}

pins! {
    SPI1:
        SCK: [
            gpioa::PA5<5>,
            gpiob::PB3<5>
        ]
        MISO: [
            gpioa::PA6<5>,
            gpiob::PB4<5>
        ]
        MOSI: [
            gpioa::PA7<5>,
            gpiob::PB5<5>
        ]

    SPI2:
        SCK: [
            gpiob::PB10<5>,
            gpiob::PB13<5>
        ]
        MISO: [
            gpiob::PB14<5>,
            gpioc::PC2<5>
        ]
        MOSI: [
            gpiob::PB15<5>,
            gpioc::PC3<5>
        ]
}

#[cfg(feature = "spi3")]
pins! {
    SPI3:
        SCK: [
            gpiob::PB3<6>,
            gpioc::PC10<6>
        ]
        MISO: [
            gpiob::PB4<6>,
            gpioc::PC11<6>
        ]
        MOSI: [
            gpiob::PB5<6>,
            gpioc::PC12<6>
        ]
}

#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI2:
        SCK: [gpiod::PD3<5>]
        MISO: []
        MOSI: []
    SPI3:
        SCK: []
        MISO: []
        MOSI: [gpiod::PD6<5>]
    SPI4:
        SCK: [
            gpioe::PE2<5>,
            gpioe::PE12<5>
        ]
        MISO: [
            gpioe::PE5<5>,
            gpioe::PE13<5>
        ]
        MOSI: [
            gpioe::PE6<5>,
            gpioe::PE14<5>
        ]
}

#[cfg(any(
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI2:
        SCK: [gpioi::PI1<5>]
        MISO: [gpioi::PI2<5>]
        MOSI: [gpioi::PI3<5>]
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446"
))]
pins! {
    SPI2:
        SCK: [gpioc::PC7<5>]
        MISO: []
        MOSI: []
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pins! {
    SPI5:
        SCK: [
            gpiob::PB0<6>
        ]
        MISO: [
            gpioa::PA12<6>
        ]
        MOSI: [
            gpioa::PA10<6>,
            gpiob::PB8<6>
        ]
}

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423"
))]
pins! {
    SPI3:
        SCK: [gpiob::PB12<7>]
        MISO: []
        MOSI: []
    SPI4:
        SCK: [gpiob::PB13<6>]
        MISO: [gpioa::PA11<6>]
        MOSI: [gpioa::PA1<5>]
    SPI5:
        SCK: [
            gpioe::PE2<6>,
            gpioe::PE12<6>
        ]
        MISO: [
            gpioe::PE5<6>,
            gpioe::PE13<6>
        ]
        MOSI: [
            gpioe::PE6<6>,
            gpioe::PE14<6>
        ]
}

#[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
pins! {
    SPI2:
        SCK: [gpioa::PA9<5>]
        MISO: [gpioa::PA12<5>]
        MOSI: [gpioa::PA10<5>]
}

#[cfg(any(
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f469",
    feature = "stm32f479"
))]
pins! {
    SPI5:
        SCK: [
            gpiof::PF7<5>,
            gpioh::PH6<5>
        ]
        MISO: [
            gpiof::PF8<5>,
            gpioh::PH7<5>
        ]
        MOSI: [
            gpiof::PF9<5>,
            gpiof::PF11<5>
        ]

    SPI6:
        SCK: [
            gpiog::PG13<5>
        ]
        MISO: [
            gpiog::PG12<5>
        ]
        MOSI: [
            gpiog::PG14<5>
        ]
}

#[cfg(any(feature = "stm32f446"))]
pins! {
    SPI2:
        SCK: [gpioa::PA9<5>]
        MISO: []
        MOSI: [gpioc::PC1<7>]

    SPI3:
        SCK: []
        MISO: []
        MOSI: [
            gpiob::PB0<7>,
            gpiob::PB2<7>,
            gpiod::PD0<6>
        ]

    SPI4:
        SCK: [gpiog::PG11<6>]
        MISO: [
            gpiog::PG12<6>,
            gpiod::PD0<5>
        ]
        MOSI: [gpiog::PG13<6>]
}

#[cfg(any(feature = "stm32f469", feature = "stm32f479"))]
pins! {
    SPI2:
        SCK: [gpioa::PA9<5>]
        MISO: []
        MOSI: [gpioc::PC1<5>]
}

/// Interrupt events
pub enum Event {
    /// New data has been received
    Rxne,
    /// Data can be sent
    Txe,
    /// An error occurred
    Error,
}

/// Normal mode - RX and TX pins are independent
pub struct TransferModeNormal;
/// BIDI mode - use TX pin as RX then spi receive data
pub struct TransferModeBidi;

#[derive(Debug)]
pub struct Spi<SPI, PINS, TRANSFER_MODE> {
    spi: SPI,
    pins: PINS,
    transfer_mode: TRANSFER_MODE,
}

// Implemented by all SPI instances
pub trait Instance:
    crate::Sealed + Deref<Target = spi1::RegisterBlock> + rcc::Enable + rcc::Reset + rcc::GetBusFreq
{
}

// Implemented by all SPI instances
macro_rules! spi {
    ($SPI:ident: ($spi:ident)) => {
        impl Instance for $SPI {}

        impl<SCK, MISO, MOSI, const SCKA: u8, const MISOA: u8, const MOSIA: u8>
            Spi<$SPI, (SCK, MISO, MOSI), TransferModeNormal>
        where
            SCK: PinSck<$SPI, A = Const<SCKA>> + SetAlternate<SCKA>,
            MISO: PinMiso<$SPI, A = Const<MISOA>> + SetAlternate<MISOA>,
            MOSI: PinMosi<$SPI, A = Const<MOSIA>> + SetAlternate<MOSIA>,
        {
            #[deprecated(since = "0.10.0", note = "Please use new instead")]
            pub fn $spi(
                spi: $SPI,
                pins: (SCK, MISO, MOSI),
                mode: Mode,
                freq: impl Into<Hertz>,
                clocks: Clocks,
            ) -> Spi<$SPI, (SCK, MISO, MOSI), TransferModeNormal> {
                Self::new(spi, pins, mode, freq, clocks)
            }
        }
    };
}

spi! { SPI1: (spi1) }
spi! { SPI2: (spi2) }

#[cfg(feature = "spi3")]
spi! { SPI3: (spi3) }

#[cfg(feature = "spi4")]
spi! { SPI4: (spi4) }

#[cfg(feature = "spi5")]
spi! { SPI5: (spi5) }

#[cfg(feature = "spi6")]
spi! { SPI6: (spi6) }

impl<SPI, SCK, MISO, MOSI, const SCKA: u8, const MISOA: u8, const MOSIA: u8>
    Spi<SPI, (SCK, MISO, MOSI), TransferModeNormal>
where
    SPI: Instance,
    SCK: PinSck<SPI, A = Const<SCKA>> + SetAlternate<SCKA>,
    MISO: PinMiso<SPI, A = Const<MISOA>> + SetAlternate<MISOA>,
    MOSI: PinMosi<SPI, A = Const<MOSIA>> + SetAlternate<MOSIA>,
{
    pub fn new(
        spi: SPI,
        mut pins: (SCK, MISO, MOSI),
        mode: Mode,
        freq: impl Into<Hertz>,
        clocks: Clocks,
    ) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        pins.0.set_alt_mode();
        pins.1.set_alt_mode();
        pins.2.set_alt_mode();

        Spi {
            spi,
            pins,
            transfer_mode: TransferModeNormal,
        }
        .pre_init(mode, freq.into(), SPI::get_frequency(&clocks))
        .init()
    }

    pub fn to_bidi_transfer_mode(self) -> Spi<SPI, (SCK, MISO, MOSI), TransferModeBidi> {
        let mut dev_w_new_t_mode = self.into_mode(TransferModeBidi {});
        dev_w_new_t_mode.enable(false);
        dev_w_new_t_mode.init()
    }
}

impl<SPI, SCK, MISO, MOSI, const SCKA: u8, const MISOA: u8, const MOSIA: u8>
    Spi<SPI, (SCK, MISO, MOSI), TransferModeBidi>
where
    SPI: Instance,
    SCK: PinSck<SPI, A = Const<SCKA>> + SetAlternate<SCKA>,
    MISO: PinMiso<SPI, A = Const<MISOA>> + SetAlternate<MISOA>,
    MOSI: PinMosi<SPI, A = Const<MOSIA>> + SetAlternate<MOSIA>,
{
    pub fn new_bidi(
        spi: SPI,
        mut pins: (SCK, MISO, MOSI),
        mode: Mode,
        freq: impl Into<Hertz>,
        clocks: Clocks,
    ) -> Self {
        unsafe {
            // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
            let rcc = &(*RCC::ptr());
            SPI::enable(rcc);
            SPI::reset(rcc);
        }

        pins.0.set_alt_mode();
        pins.1.set_alt_mode();
        pins.2.set_alt_mode();

        Spi {
            spi,
            pins,
            transfer_mode: TransferModeBidi,
        }
        .pre_init(mode, freq.into(), SPI::get_frequency(&clocks))
        .init()
    }

    pub fn to_normal_transfer_mode(self) -> Spi<SPI, (SCK, MISO, MOSI), TransferModeNormal> {
        let mut dev_w_new_t_mode = self.into_mode(TransferModeNormal {});
        dev_w_new_t_mode.enable(false);
        dev_w_new_t_mode.init()
    }
}

impl<SPI, SCK, MISO, MOSI, TRANSFER_MODE, const SCKA: u8, const MISOA: u8, const MOSIA: u8>
    Spi<SPI, (SCK, MISO, MOSI), TRANSFER_MODE>
where
    SPI: Instance,
    SCK: PinSck<SPI, A = Const<SCKA>> + SetAlternate<SCKA>,
    MISO: PinMiso<SPI, A = Const<MISOA>> + SetAlternate<MISOA>,
    MOSI: PinMosi<SPI, A = Const<MOSIA>> + SetAlternate<MOSIA>,
{
    pub fn release(mut self) -> (SPI, (SCK, MISO, MOSI)) {
        self.pins.0.restore_mode();
        self.pins.1.restore_mode();
        self.pins.2.restore_mode();

        (self.spi, self.pins)
    }

    #[deprecated(since = "0.10.0", note = "Please use release instead")]
    pub fn free(self) -> (SPI, (SCK, MISO, MOSI)) {
        self.release()
    }
}

impl<SPI, PINS> Spi<SPI, PINS, TransferModeNormal>
where
    SPI: Instance,
{
    pub fn init(self) -> Self {
        self.spi.cr1.modify(|_, w| {
            // bidimode: 2-line unidirectional
            w.bidimode()
                .clear_bit()
                .bidioe()
                .clear_bit()
                // spe: enable the SPI bus
                .spe()
                .set_bit()
        });

        self
    }
}

impl<SPI, PINS> Spi<SPI, PINS, TransferModeBidi>
where
    SPI: Instance,
{
    pub fn init(self) -> Self {
        self.spi.cr1.modify(|_, w| {
            // bidimode: 1-line unidirectional
            w.bidimode()
                .set_bit()
                .bidioe()
                .set_bit()
                // spe: enable the SPI bus
                .spe()
                .set_bit()
        });

        self
    }
}

impl<SPI, PINS, TRANSFER_MODE> Spi<SPI, PINS, TRANSFER_MODE>
where
    SPI: Instance,
{
    /// Convert the spi to another transfer mode.
    fn into_mode<TRANSFER_MODE2>(
        self,
        transfer_mode: TRANSFER_MODE2,
    ) -> Spi<SPI, PINS, TRANSFER_MODE2> {
        Spi {
            spi: self.spi,
            pins: self.pins,
            transfer_mode,
        }
    }

    /// Enable/disable spi
    pub fn enable(&mut self, enable: bool) {
        self.spi.cr1.modify(|_, w| {
            // spe: enable the SPI bus
            w.spe().bit(enable)
        });
    }

    /// Pre initializing the SPI bus.
    pub fn pre_init(self, mode: Mode, freq: Hertz, clock: Hertz) -> Self {
        // disable SS output
        self.spi.cr2.write(|w| w.ssoe().clear_bit());

        let br = match clock.0 / freq.0 {
            0 => unreachable!(),
            1..=2 => 0b000,
            3..=5 => 0b001,
            6..=11 => 0b010,
            12..=23 => 0b011,
            24..=47 => 0b100,
            48..=95 => 0b101,
            96..=191 => 0b110,
            _ => 0b111,
        };

        self.spi.cr1.write(|w| {
            w.cpha()
                .bit(mode.phase == Phase::CaptureOnSecondTransition)
                .cpol()
                .bit(mode.polarity == Polarity::IdleHigh)
                // mstr: master configuration
                .mstr()
                .set_bit()
                .br()
                .bits(br)
                // lsbfirst: MSB first
                .lsbfirst()
                .clear_bit()
                // ssm: enable software slave management (NSS pin free for other uses)
                .ssm()
                .set_bit()
                // ssi: set nss high = master mode
                .ssi()
                .set_bit()
                .rxonly()
                .clear_bit()
                // dff: 8 bit frames
                .dff()
                .clear_bit()
        });

        self
    }

    /// Enable interrupts for the given `event`:
    ///  - Received data ready to be read (RXNE)
    ///  - Transmit data register empty (TXE)
    ///  - Transfer error
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::Rxne => self.spi.cr2.modify(|_, w| w.rxneie().set_bit()),
            Event::Txe => self.spi.cr2.modify(|_, w| w.txeie().set_bit()),
            Event::Error => self.spi.cr2.modify(|_, w| w.errie().set_bit()),
        }
    }

    /// Disable interrupts for the given `event`:
    ///  - Received data ready to be read (RXNE)
    ///  - Transmit data register empty (TXE)
    ///  - Transfer error
    pub fn unlisten(&mut self, event: Event) {
        match event {
            Event::Rxne => self.spi.cr2.modify(|_, w| w.rxneie().clear_bit()),
            Event::Txe => self.spi.cr2.modify(|_, w| w.txeie().clear_bit()),
            Event::Error => self.spi.cr2.modify(|_, w| w.errie().clear_bit()),
        }
    }

    /// Return `true` if the TXE flag is set, i.e. new data to transmit
    /// can be written to the SPI.
    pub fn is_txe(&self) -> bool {
        self.spi.sr.read().txe().bit_is_set()
    }

    /// Return `true` if the RXNE flag is set, i.e. new data has been received
    /// and can be read from the SPI.
    pub fn is_rxne(&self) -> bool {
        self.spi.sr.read().rxne().bit_is_set()
    }

    /// Return `true` if the MODF flag is set, i.e. the SPI has experienced a
    /// Master Mode Fault. (see chapter 28.3.10 of the STM32F4 Reference Manual)
    pub fn is_modf(&self) -> bool {
        self.spi.sr.read().modf().bit_is_set()
    }

    /// Return `true` if the OVR flag is set, i.e. new data has been received
    /// while the receive data register was already filled.
    pub fn is_ovr(&self) -> bool {
        self.spi.sr.read().ovr().bit_is_set()
    }

    #[inline(always)]
    fn check_read(&mut self) -> nb::Result<u8, Error> {
        let sr = self.spi.sr.read();

        Err(if sr.ovr().bit_is_set() {
            Error::Overrun.into()
        } else if sr.modf().bit_is_set() {
            Error::ModeFault.into()
        } else if sr.crcerr().bit_is_set() {
            Error::Crc.into()
        } else if sr.rxne().bit_is_set() {
            return Ok(self.read_u8());
        } else {
            nb::Error::WouldBlock
        })
    }

    #[inline(always)]
    fn check_send(&mut self, byte: u8) -> nb::Result<(), Error> {
        let sr = self.spi.sr.read();

        Err(if sr.ovr().bit_is_set() {
            // Read from the DR to clear the OVR bit
            let _ = self.spi.dr.read();
            Error::Overrun.into()
        } else if sr.modf().bit_is_set() {
            // Write to CR1 to clear MODF
            self.spi.cr1.modify(|_r, w| w);
            Error::ModeFault.into()
        } else if sr.crcerr().bit_is_set() {
            // Clear the CRCERR bit
            self.spi.sr.modify(|_r, w| {
                w.crcerr().clear_bit();
                w
            });
            Error::Crc.into()
        } else if sr.txe().bit_is_set() {
            self.send_u8(byte);
            return Ok(());
        } else {
            nb::Error::WouldBlock
        })
    }

    #[inline(always)]
    fn read_u8(&mut self) -> u8 {
        // NOTE(read_volatile) read only 1 byte (the svd2rust API only allows reading a half-word)
        unsafe { ptr::read_volatile(&self.spi.dr as *const _ as *const u8) }
    }

    #[inline(always)]
    fn send_u8(&mut self, byte: u8) {
        // NOTE(write_volatile) see note above
        unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut u8, byte) }
    }
}

impl<SPI, PINS> spi::FullDuplex<u8> for Spi<SPI, PINS, TransferModeNormal>
where
    SPI: Instance,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        self.check_read()
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
        self.check_send(byte)
    }
}

impl<SPI, PINS> spi::FullDuplex<u8> for Spi<SPI, PINS, TransferModeBidi>
where
    SPI: Instance,
{
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        self.spi.cr1.modify(|_, w| w.bidioe().clear_bit());
        self.check_read()
    }

    fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
        self.spi.cr1.modify(|_, w| w.bidioe().set_bit());
        self.check_send(byte)
    }
}

mod blocking {
    use super::{Error, Instance, Spi, TransferModeBidi, TransferModeNormal};
    use embedded_hal::blocking::spi::{Operation, Transactional, Transfer, Write, WriteIter};
    use embedded_hal::spi::FullDuplex;

    impl<SPI, PINS, TRANSFER_MODE> Transfer<u8> for Spi<SPI, PINS, TRANSFER_MODE>
    where
        Self: FullDuplex<u8, Error = Error>,
        SPI: Instance,
    {
        type Error = Error;

        fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
            for word in words.iter_mut() {
                nb::block!(self.send(*word))?;
                *word = nb::block!(self.read())?;
            }

            Ok(words)
        }
    }

    impl<SPI, PINS> Write<u8> for Spi<SPI, PINS, TransferModeNormal>
    where
        Self: FullDuplex<u8, Error = Error>,
        SPI: Instance,
    {
        type Error = Error;

        fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            for word in words {
                nb::block!(self.send(*word))?;
                nb::block!(self.read())?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS> Write<u8> for Spi<SPI, PINS, TransferModeBidi>
    where
        Self: FullDuplex<u8, Error = Error>,
        SPI: Instance,
    {
        type Error = Error;

        fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
            for word in words {
                nb::block!(self.send(*word))?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS> WriteIter<u8> for Spi<SPI, PINS, TransferModeNormal>
    where
        Self: FullDuplex<u8, Error = Error>,
        SPI: Instance,
    {
        type Error = Error;

        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u8>,
        {
            for word in words.into_iter() {
                nb::block!(self.send(word))?;
                nb::block!(self.read())?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS> WriteIter<u8> for Spi<SPI, PINS, TransferModeBidi>
    where
        Self: FullDuplex<u8, Error = Error>,
        SPI: Instance,
    {
        type Error = Error;

        fn write_iter<WI>(&mut self, words: WI) -> Result<(), Self::Error>
        where
            WI: IntoIterator<Item = u8>,
        {
            for word in words.into_iter() {
                nb::block!(self.send(word))?;
            }

            Ok(())
        }
    }

    impl<SPI, PINS, TRANSFER_MODE, W: 'static> Transactional<W> for Spi<SPI, PINS, TRANSFER_MODE>
    where
        Self: Write<W, Error = Error> + Transfer<W, Error = Error>,
    {
        type Error = Error;

        fn exec<'a>(&mut self, operations: &mut [Operation<'a, W>]) -> Result<(), Error> {
            for op in operations {
                match op {
                    Operation::Write(w) => self.write(w)?,
                    Operation::Transfer(t) => self.transfer(t).map(|_| ())?,
                }
            }

            Ok(())
        }
    }
}

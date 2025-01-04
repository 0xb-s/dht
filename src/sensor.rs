use crate::constants::{SensorType, MAX_CYCLES};
use crate::error::DhtError;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};

/// Struct representing a DHT sensor
pub struct Dht<PIN, DELAY> {
    pin: PIN,
    delay: DELAY,
    sensor_type: SensorType,
}
impl<PIN, DELAY> Dht<PIN, DELAY>
where
    PIN: OutputPin + InputPin,
    DELAY: DelayNs,
{
    /// Creates a new DHT sensor instance

    pub fn new(pin: PIN, delay: DELAY, sensor_type: SensorType) -> Self {
        Dht {
            pin,
            delay,
            sensor_type,
        }
    }

    /// Initializes the DHT sensor

    pub fn begin(&mut self) -> Result<(), DhtError> {
        
        self.pin.set_high().map_err(|_| DhtError::IoError)?;
   
        self.delay.delay_us(20_000); // 20 ms
        Ok(())
    }

    /// Sends the start signal to the DHT sensor
    fn send_start_signal(&mut self) -> Result<(), DhtError> {
 
        self.pin.set_low().map_err(|_| DhtError::IoError)?;

     
        self.delay.delay_us(self.sensor_type.signal_pulse_us());

   
        self.pin.set_high().map_err(|_| DhtError::IoError)?;

     
        self.delay.delay_us(40); // 40 us

        Ok(())
    }

    /// Expects a pulse at a given level and measures its duration
    ///
    ///
    fn expect_pulse(&mut self, level: bool) -> Result<u32, DhtError> {
        let mut count: u32 = 0;

        loop {
            let pin_state = if level {
                self.pin.is_high()
            } else {
                self.pin.is_low()
            };

            match pin_state {
                Ok(state) => {
                    if state != level {
                        break;
                    }
                }
                Err(_) => return Err(DhtError::IoError),
            }

            count += 1;
            if count > MAX_CYCLES {
                return Err(DhtError::Timeout);
            }

          
            self.delay.delay_us(1);
        }

        Ok(count)
    }
    /// Validates the checksum of the received data
    fn validate_checksum(&self, bits: &[u8; 5]) -> Result<(), DhtError> {
        let checksum = bits[0]
            .wrapping_add(bits[1])
            .wrapping_add(bits[2])
            .wrapping_add(bits[3]);
        if checksum == bits[4] {
            Ok(())
        } else {
            Err(DhtError::ChecksumMismatch)
        }
    }
    /// Parses raw data into temperature and humidity
    fn parse_data(&self, bits: &[u8; 5]) -> Result<(f32, f32), DhtError> {
        let humidity: f32;
        let mut temperature: f32;

        match self.sensor_type {
            SensorType::DHT11 => {
                humidity = bits[0] as f32 + (bits[1] as f32) * 0.1;
                temperature = bits[2] as f32 + (bits[3] as f32) * 0.1;
                //remember that min valeu is 0
            }
            SensorType::DHT22 => {
                humidity = ((bits[0] as u16) << 8 | bits[1] as u16) as f32 * 0.1;
                temperature = ((bits[2] as u16 & 0x7F) << 8 | bits[3] as u16) as f32 * 0.1;
                if bits[2] & 0x80 != 0 {
                    // Negative temperature
                    temperature = -temperature;
                }
            }
        }

        Ok((temperature, humidity))
    }

    /// Converts Celsius to Fahrenheit
    pub fn convert_c_to_f(&self, c: f32) -> f32 {
        c * 1.8 + 32.0
    }

    /// Converts Fahrenheit to Celsius
    pub fn convert_f_to_c(&self, f: f32) -> f32 {
        (f - 32.0) * 5.0 / 9.0
    }

    /// Reads temperature and humidity from the sensor

    pub fn read(&mut self) -> Result<(f32, f32), DhtError> {
    
        self.send_start_signal()?;

      
        let bits = self.read_raw_data()?;

     
        self.validate_checksum(&bits)?;

       
        let (humidity, temperature) = self.parse_data(&bits)?;

        Ok((temperature, humidity))
    }

    /// Reads raw bits from the sensor
    fn read_raw_data(&mut self) -> Result<[u8; 5], DhtError> {
        let mut bits: [u8; 5] = [0; 5];

      
        self.expect_pulse(false)?;
        self.expect_pulse(true)?;


        for i in 0..40 {
         
            self.expect_pulse(false)?;

       
            let duration = self.expect_pulse(true)?;

       
            if duration > 50 {
                bits[i / 8] |= 1 << (7 - (i % 8));
            }
        }

        Ok(bits)
    }

   
}

extern crate leap_year ;
use leap_year::bissextile;
use std::time::{SystemTime, UNIX_EPOCH} ;
use std::env ;

pub mod todayte {
    use std::fmt ;
    use std::fmt::{Display, Formatter} ;

    const SEQ_MES: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31] ;
    const ANO_NORMAL: u64 = 31536000 ;
    const ANO_BISSXT: u64 = 31622400 ;
    const DIA_SECS: u64 = 86400 ;
    const HOR_SECS: u64 = 3600 ;
    const  MIN_SECS: u64 = 60 ;

    #[derive(Debug)]
    pub struct Todayte {
        ano: u16 ,
        mes: u8 ,
        dia: u8 ,    
        hor: i8 ,
        min: u8 ,
        seg: u8 ,
    }

    impl Todayte {
        pub fn new(mut secs: u64, gmt: i8) -> Todayte {
            let mut ano: u16 = 1970 ;
            let mut mes: u8 = 1 ;
            let mut dia: u8 = 1 ;
            let mut hor: i8 = gmt ;
            let mut min: u8 = 0 ;
            let seg: u8 ;

            let mut idxmss: usize = 0 ;

            while secs > MIN_SECS {
                let is_bissext: bool = leap_year::bissextile(ano) ;
                let mut qtd_days_month: u64 ;
                if is_bissext {
                    qtd_days_month = ANO_BISSXT  ;
                } else {
                    qtd_days_month = ANO_NORMAL ;
                }
                if secs > qtd_days_month {
                    secs -= qtd_days_month ;
                    ano += 1 ;
                } else {
                    qtd_days_month = DIA_SECS * SEQ_MES[idxmss] as u64 + (is_bissext as u64 * DIA_SECS) ;
                    if secs > qtd_days_month {
                        secs -= qtd_days_month ;
                        mes += 1 ;
                        idxmss += 1 ;
                        if mes > 12 {
                            ano += 1 ;
                            mes = 1 ;
                            idxmss = 0 ;
                        }
                    } else if secs > DIA_SECS + (is_bissext as u64) {
                        secs -= DIA_SECS ;
                        dia += 1 ;
                        if dia > SEQ_MES[idxmss] {
                            mes += 1 ;
                            idxmss += 1 ;
                            if mes > 12 {
                                ano += 1 ;
                                mes = 1 ;
                                dia = 1 ;
                                idxmss = 0 ;
                            }
                        }
                    } else if secs > HOR_SECS {
                        secs -= HOR_SECS ;
                        hor += 1 ;
                        if hor > 23 {
                            hor = 0 ;
                            dia += 1 ;
                            if dia > SEQ_MES[idxmss] + (is_bissext as u8) {
                                mes += 1 ;
                                idxmss += 1 ;
                                if mes > 12 {
                                    ano += 1 ;
                                    mes = 1 ;
                                    dia = 1 ;
                                    idxmss = 0 ;
                                }
                            }
                        }
                    } else if secs > MIN_SECS as u64 {
                    secs -= MIN_SECS as u64;
                    min += 1 ;
                    if min > 59 {
                        min = 0 ;
                        hor += 1 ;
                        if hor > 23 {
                            hor = 0 ;
                            dia += 1 ;
                            if dia > SEQ_MES[idxmss] + (is_bissext as u8) {
                                dia = 1 ;
                                mes += 1 ;
                                idxmss += 1 ;
                                if mes > 12 {
                                    idxmss = 0 ;
                                    mes = 1;
                                    ano += 1 ;
                                }
                            }
                        }
                    }
                }
            }
        }
        seg = secs as u8 ;
        Todayte { ano, mes, dia, hor, min, seg }
        }
    }
    
    impl Display for Todayte {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{:04}-{:02}-{:02} {:02}:{:02}:{:02}", self.ano, self.mes, self.dia, self.hor, self.min, self.seg) ;
                Ok(()) 
        }
    }
}

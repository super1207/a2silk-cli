// use std::io::BufReader;

// use super::all_to_silk::PCMStruct;

// pub fn deal_mp3(bufr: BufReader<&[u8]>)  -> Result<PCMStruct, Box<dyn std::error::Error>> {
//     let mut pcm = PCMStruct{
//         channel_num:1,
//         bits_per_sample:16,
//         sample_rate:24000,
//         data: Vec::new(),
//     };
    
//     let mut buf = vec![];
//     std::io::Read::read_to_end(&mut BufReader::new(bufr), &mut buf)?;
//     let decoder = rodio::Decoder::new(std::io::Cursor::new(buf))?;
//     pcm.sample_rate = rodio::Source::sample_rate(&decoder) as usize;
//     pcm.channel_num = rodio::Source::channels(&decoder) as usize;
//     for it in decoder {
//         pcm.data.push(it as f64);
//     }
//     return Ok(pcm);
// }
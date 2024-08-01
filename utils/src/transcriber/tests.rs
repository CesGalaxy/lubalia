use super::*;

#[test]
fn test_transcriber() {
    let source = vec![1, 2, 3, 4, 5];
    let result = transcriber::<u8, u8, &str>(source, |_, unit| {
        if *unit == 3 {
            Ok(Some(*unit))
        } else {
            Ok(None)
        }
    });

    assert_eq!(result.unwrap().units(), vec![&3]);
}

#[test]
fn test_transcriber_error() {
    let source = vec![1, 2, 3, 4, 5];
    let result = transcriber::<u8, u8, &str>(source, |cursor, _| {
        cursor.next();
        cursor.next();

        Err(TranscriptionException::Error("error"))
    });

    assert_eq!(result.unwrap_err().tick_buffer, vec![1, 2]);
}

#[test]
fn test_transcriber_units() {
    let source = vec![1, 2, 3, 4, 5];
    let result = transcriber::<u8, u8, &str>(source, |_, unit| {
        if *unit == 3 {
            Ok(Some(*unit))
        } else {
            Ok(None)
        }
    });

    assert_eq!(result.unwrap().units(), vec![&3]);
}

#[test]
fn test_transcriber_units_position() {
    let source = vec![1, 2, 3, 4, 5];
    let transcription = transcriber::<u8, u8, &str>(source, |cursor, unit| {
        if *unit == 3 {
            cursor.next();
            Ok(Some(*unit))
        } else {
            Ok(None)
        }
    });

    let transcription = transcription.expect("transcription should be successful");

    assert_eq!(transcription.result[0].source_position, Some(2));
    assert_eq!(transcription.result[0].source_length, Some(1));
}

#[test]
fn test_transcriber_no_transcription() {
    let source = vec![1, 2, 3, 4, 5];
    let result = transcriber::<u8, u8, &str>(source, |_, _| Ok(None));

    assert_eq!(result.unwrap().units(), vec![&0; 0]);
}

#[test]
fn test_transcriber_multiple_transcriptions() {
    let source = vec![1, 2, 3, 4, 5];
    let result = transcriber::<u8, u8, &str>(source, |_, unit| {
        if *unit == 2 || *unit == 4 {
            Ok(Some(*unit))
        } else {
            Ok(None)
        }
    });

    assert_eq!(result.unwrap().units(), vec![&2, &4]);
}

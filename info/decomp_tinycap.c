// file.wav [-D card] [-d device] [-c channels] [-r rate] [-b bits] [-p
// period_size] [-n n_periods] [-t duration] [-f]
int __fastcall sub_E80(int filename, int card, int device, int channels,
                       int rate, int format, int period_size, int n_periods,
                       int duration) {
  int pcm;                   // r0
  int pcm;                   // r9
  int frames;                // r0
  size_t buffer_size;        // r7
  void *buf;                 // r6
  unsigned int max_filesize; // r10
  int bits;                  // r0
  unsigned int i;            // r5
  int read_result;           // r0
  int frames_recorded;       // r5
  FILE *v22;                 // r5
  const char *error;         // r0

  pcm = pcm_open(card, device, 0x10000000);
  if (pcm && pcm_is_ready(pcm)) {
    frames = pcm_get_buffer_size(pcm);
    buffer_size = pcm_frames_to_bytes(pcm, frames);
    buf = malloc(buffer_size);
    if (buf) {
      if (duration)
        max_filesize = pcm_frames_to_bytes(pcm, duration * rate) - 1;
      else
        max_filesize = -1;

      bits = pcm_format_to_bits(format);
      printf("Capturing sample: %u ch, %u hz, %u bit\n", channels, rate, bits);
      for (i = 0; dword_5000; i += buffer_size) {
        read_result = pcm_read(pcm, buf, buffer_size);
        if (max_filesize < i || read_result)
          break;
        if (_fwrite_chk(buf, 1, buffer_size, filename, -1) != v14) {
          _fwrite_chk("Error capturing sample\n", 23, 1, stderr, 24);
          break;
        }
      }
      frames_recorded = pcm_bytes_to_frames(pcm, i);
      pcm_close(pcm);
      free(buf);
    } else {
      fprintf(stderr, "Unable to allocate %d bytes\n", buffer_size);
      pcm_close(pcm);
      return 0;
    }
  } else {
    v22 = stderr;
    error = (const char *)pcm_get_error(pcm);
    fprintf(v22, "Unable to open PCM device (%s)\n", error);
    return 0;
  }
  return frames_recorded;
}

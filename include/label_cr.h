#ifndef LABEL_CR_H
#define LABEL_CR_H

#include "type_font.h"

typedef struct LabelCr LabelCROpaque;

extern LabelCROpaque* label_cr_new(const char *path, double width, double height);
extern void label_cr_set_param_args(LabelCROpaque *ptr, const char *font_family, double size, FontSlantCr slant, FontWeightCr weight, double x, double y);

#endif
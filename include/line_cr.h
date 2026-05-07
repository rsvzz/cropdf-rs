#ifndef LINE_CR_H
#define LINE_CR_H

#include "axis_cr.h"

typedef struct LineCr LineCROpaque;

extern LineCROpaque* line_cr_new(double width, double height, double thickness, AxisCr axis);
extern void line_cr_set_param_args(LineCROpaque *ptr, double x, double y);

#endif
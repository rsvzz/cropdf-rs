#ifndef CREATE_PDF_H
#define CREATE_PDF_H

#include "label_cr.h"

typedef struct CreatePDF CreatePDFOpaque;


CreatePDFOpaque* create_pdf_new(const char *path, double width, double height);

extern void create_pdf_surface(CreatePDFOpaque* pdf);
extern void create_pdf_drop(CreatePDFOpaque* pdf);
extern void create_pdf_add(CreatePDFOpaque* pdf, LabelCROpaque* label);
extern void create_pdf_new_page(CreatePDFOpaque* pdf);

extern const char * create_pdf_get_path(CreatePDFOpaque * pdf);

extern void create_pdf_free(CreatePDFOpaque* pdf);

#endif
#include "include/create_pdf.h"
#include "include/label_cr.h"
#include "include/line_cr.h"

#include "stdio.h"

int main(void){
  // test lib
  // cargo build  --release
  // gcc main.c -o main -lcropdf -L target/release
  // LD_LIBRARY_PATH=target/release ./main        

  CreatePDFOpaque *pdf = create_pdf_new("report.pdf", 595.0, 842.0);
  LabelCROpaque *lbl_title = label_cr_new("My title report", 50.0, 50.0);
  LineCROpaque *line = line_cr_new(10,400, 1.0, AXIS_VERTITAL);

  label_cr_set_param_args(lbl_title, "Sans", 14, SLANT_NORMAL, WEIGHT_BOLD, 50.0, 50.0);
  line_cr_set_param_args(line, 50.0, 60.0);

  create_pdf_surface(pdf);
  create_pdf_add(pdf, lbl_title); //draw title
  create_pdf_add_line(pdf, line); //draw line
  create_pdf_new_page(pdf);
  create_pdf_drop(pdf);
  
  const char *path = create_pdf_get_path(pdf);
  create_pdf_free(pdf);
    return 0;
}
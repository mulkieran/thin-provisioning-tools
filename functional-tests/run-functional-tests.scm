(import (rnrs)
        (fmt fmt)
        (functional-tests)
        (cache-functional-tests)
        (only (srfi s1 lists) break)
        (srfi s8 receive)
        (thin-functional-tests))

;;------------------------------------------------

(define (begins-with prefix xs)
  (cond
    ((null? prefix) #t)
    ((null? xs) #f)
    ((eq? (car prefix) (car xs))
     (begins-with (cdr prefix) (cdr xs)))
    (else #f)))

(define (split-list xs sep)
  (define (safe-cdr xs)
    (if (null? xs) '() (cdr xs)))

  (if (null? xs)
      '()
      (receive (p r) (break (lambda (c)
                          (eq? c sep))
                        xs)
           (cons p (split-list (safe-cdr r) sep)))))

(define (string->syms str sep)
  (map (lambda (cs)
         (string->symbol
           (list->string cs)))
       (split-list (string->list str) sep)))

(define (mk-filter pattern)
  (let ((prefix (string->syms pattern #\/)))
   (lambda (keys)
    (begins-with prefix keys))))

;;------------------------------------------------

(register-thin-tests)
(register-cache-tests)

(if (run-scenarios (filter (mk-filter (car (cdr (command-line))))
                           (list-scenarios)))
    (exit)
    (exit #f))


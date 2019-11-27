
(if (> age 18)
        "adult"
    "underage")

(match name
    ("x" 
        (progn
            (somthing)))
    (_ 
        (progn
            (something-2))))

(defun something ()
    (print "")
    (print "yo")
    (print "x"))

(defvar numbers 
    (list 1 2 3))


(loop i v in numbers 
    (print i))

(defvar fn 
    (lambda 
        (x r)
        (print x)))
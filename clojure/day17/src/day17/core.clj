(ns day17.core)

(defn trajectory-x
  ([x] (lazy-seq (cons x (trajectory-x (if (> x 0) (dec x) 0))))))

(defn trajectory-y
  ([y] (lazy-seq (cons y (trajectory-y (dec y))))))

(defn trajectory
  [x y]
  (lazy-seq (partition 2 (interleave (reductions + (trajectory-x x)) (reductions + (trajectory-y y))))))

(defn within-target?
  [x y]
  (< 0 (count
         (filter (fn [[x y :as %]] (and (>= x 85)
                                        (<= x 145)
                                        (<= y -108)
                                        (>= y -163)))
         ;(filter (fn [[x y :as %]] (and (>= x 20)
         ;                               (<= x 30)
         ;                               (<= y -5)
         ;                               (>= y -10)))
                 (take 1000 (trajectory x y))))))

;(def l (cart [(range 10 145) (range 1 1000)]))
;(apply max-key second (filter (fn [[x y :as %]] (within-target? x y)) l))
; to get the highest y value:
;(second (apply max-key second (take 1000 (trajectory 16 162))))
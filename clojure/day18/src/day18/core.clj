(ns day18.core)

(defrecord Pair [left right])

(defn magnitude
  [{:keys [left right] :as pair}]
  (cond
    (and (number? left) (number? right)) (+ (* left 3) (* right 2))
    (number? left) (+ (* left 3) (* 2 (magnitude right)))
    (number? right) (+ (* 3 (magnitude left)) (* right 2))
    :else (+ (* 3 (magnitude left)) (* 2 (magnitude right)))))

(defn height
  ([tree] (height tree 0))
  ([tree count]
   (if tree
     (max (height (:left tree) (inc count))
          (height (:right tree) (inc count)))
     count)))

(defn add-to-right
  [t v]
  (if (number? t) (+ t v) (->Pair (add-to-right (:left t) v) (:right t))))

(defn add-to-left
  [t v]
  (if (number? t) (+ t v) (->Pair (:left t) (add-to-left (:right t) v))))

(defn left-of-exploding-pair
  [t h]
  (if (= h 0) true
      (if (> (height (:left t)) h) (left-of-exploding-pair (:left t) (dec h)) false)))

(defn send-left
  "Add exploding pair's left value to the first regular number to the left of the exploding pair"
  ([pair val] (send-left pair val 4))
  ([{:keys [left right] :as pair} val count]
   (cond
     (= count 0) pair
     (> (height left) count) (->Pair (send-left left val (dec count)) right)
     (> (height right) count) (if (left-of-exploding-pair right (dec count))
                                (->Pair (add-to-left left val) right)
                                (->Pair left (send-left right val (dec count))))
     :else pair)))

(defn right-of-exploding-pair
  [{:keys [left right] :as pair} count]
  (cond
    (= count 0) true
    (> (height left) count) false
    :else (right-of-exploding-pair right (dec count))))

(defn send-right
  "Add exploding pair's right value to the first regular number to the right of the exploding pair"
  ([pair val] (send-right pair val 4))
  ([{:keys [left right] :as pair} val count]
   (cond
     (= count 0) pair
     (> (height left) count) (if (right-of-exploding-pair left (dec count))
                               (->Pair left (add-to-right right val))
                               (->Pair (send-right left val (dec count)) right))
     (> (height right) count) (->Pair left (send-right right val (dec count)))
     :else pair)))

(defn replace-with-zero
  "Replace leftmost exploding pair with a regular number 0"
  ([pair] (replace-with-zero pair 4))
  ([{:keys [left right] :as pair} count]
   (cond
     (= count 0) 0
     (> (height left) count) (->Pair (replace-with-zero left (dec count)) right)
     (> (height right) count) (->Pair left (replace-with-zero right (dec count)))
     :else pair)))

(defn find-exploding-pair
  "Returns leftmost exploding pair if one exists, on nil if none found"
  ([pair] (find-exploding-pair pair 4))
  ([{:keys [left right] :as pair} count]
   (cond
     (= count 0) pair
     (> (height left) count) (find-exploding-pair left (dec count))
     (> (height right) count) (find-exploding-pair right (dec count))
     :else nil)))

(defn explode
  ([pair] (explode pair 4))
  ([{:keys [left right] :as pair} count]
   (let [pair-to-explode (find-exploding-pair pair)]
     (if pair-to-explode
       (-> pair
           (send-right (:right pair-to-explode))
           (send-left (:left pair-to-explode))
           (replace-with-zero))
       pair))))

(defn add [n1 n2]
  (cond
    (and (number? n1) (number? n2)) (Pair. n1 n2)
    (number? n1) (Pair. n1 (add (first n2) (second n2)))
    (number? n2) (Pair. (add (first n1) (second n1)) n2)
    :else (Pair. (add (first n1) (second n1)) (add (first n2) (second n2)))))

(defn add-pairs [p1 p2]
  (->Pair p1 p2))

(def to-tree #(reduce add %))

(defn split [n]
  (let [q (quot n 2)
        r (rem n 2)]
    (->Pair q (if (= 0 r) q (inc q)))))

(defn to-seq
  [t]
  (if (number? t)
    [t]
    (concat (to-seq (:left t)) (to-seq (:right t)))))

(defn find-and-split
  [{:keys [left right] :as t}]
  (cond
    (and (number? t) (> t 9)) (split t)
    (some #(> % 9) (to-seq left)) (->Pair (find-and-split left) right)
    :else (->Pair left (find-and-split right))))

(defn sf-reduce
  [tree]
  (loop [t tree]
    (if (<= (height t) 5)
      (if (= 0 (count (filter #(> % 9) (to-seq t))))
        t
        (recur (find-and-split t)))
      (recur (explode t)))))

(def sf-add (comp sf-reduce add-pairs))

(defn cart [colls]
  (if (empty? colls)
    '(())
    (for [more (cart (rest colls))
          x (first colls)]
      (cons x more))))

(defn part1 [s]
  (magnitude (reduce sf-add (map to-tree (mapv read-string (clojure.string/split s #"\n"))))))

(defn part2 [s]
  (let [nums (map to-tree (mapv read-string (clojure.string/split s #"\n")))]
    (apply max (map #(magnitude (reduce sf-add %)) (cart (list nums nums))))))

(ns day13.core-test
  (:require [clojure.test :refer :all]
            [day13.core :refer :all]))

(deftest test-fold-up
  (testing "folding up"
    (is (= #{[3 0]} (fold-along-y ['(3 0)] 7)))
    (is (= #{[3 0] [0 0] [2 0]} (fold-along-y ['(3 0) '(0 14) '(2 14)] 7)))
    (is (= 3 (count (into (sorted-set) (fold-along-y ['(3 0) '(0 14) '(2 14) '(3 14)] 7)))))
    (is (= #{[0 0] [0 1]} (fold-along-y ['(0 14) '(0 13)] 7)))))

(deftest test-fold-left
  (testing "folding along the x acex"
    (is (= #{[0 3]} (fold-along-x ['(1310 3)] 655)))))

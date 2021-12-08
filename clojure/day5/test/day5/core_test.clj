(ns day5.core-test
  (:require [clojure.test :refer :all]
            [day5.core :refer :all]))

(deftest test-parsing
  (testing "parsing input"
    (is (= {:x1 0 :y1 9 :x2 5 :y2 9} (first (parse (get-input "sample.txt")))))
    (is (= 10 (count (parse (get-input "sample.txt")))))))

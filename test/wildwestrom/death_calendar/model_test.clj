;; Death Calendar: See how many days you have left to live at a glance.
;; Copyright © 2021 Christian Westrom

;; This program is free software: you can redistribute it and/or modify
;; it under the terms of the GNU Affero General Public License as
;; published by the Free Software Foundation, either version 3 of the
;; License, or (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU Affero General Public License for more details.

;; You should have received a copy of the GNU Affero General Public License
;; along with this program.  If not, see <https://www.gnu.org/licenses/>.

(ns wildwestrom.death-calendar.model-test
  (:require [clojure.test :refer [deftest testing is are]]
            [wildwestrom.death-calendar.model :as sut])
  (:import (java.time LocalDate Period)))

(deftest death-day
  (testing "Given a birthday and lifespan return death-day."
    (are [death-day b-day lifespan] (= death-day (sut/death-day b-day lifespan))
      (LocalDate/of 2080 1 1)  (LocalDate/of 2000 1 1)  (Period/ofYears 80)
      (LocalDate/of 2098 8 15) (LocalDate/of 1998 8 15) (Period/ofYears 100)
      (LocalDate/of 2000 2 1)  (LocalDate/of 2000 1 1)  (Period/ofMonths 1)
      (LocalDate/of 2001 1 1)  (LocalDate/of 2000 1 1)  (Period/ofDays 366))))

(defn days-generator
  []
  (rand-int (* 365.25 110)))

(defn date-generator
  []
  (.plusDays (LocalDate/now)
             (rand-nth (let [hundred-years (* 365.25 110)]
                         (range 1 hundred-years)))))

(deftest days-left-to-live
  (testing "Given a lifespan as a `java.time.Period` object, return days left."
    (dotimes [n 20]
      (let [days (days-generator)
            date (date-generator)
            test-expr #(sut/days-left date (Period/ofDays %))]
        #_(println (str "run " (inc n) ":"
                        days " days, "
                        (format "%.1f" (/ days 365.25)) " years"))
        (is days (test-expr days))))))

(deftest calendar-map
  (testing "Make sure calendar-map is valid."
    (dotimes [n 10]
     (let [test-map (sut/calendar-map (date-generator)
                                      (Period/ofDays (days-generator)))]
      (is (= (:total test-map) (+ (:lived test-map)
                              (:remaining test-map))))))))

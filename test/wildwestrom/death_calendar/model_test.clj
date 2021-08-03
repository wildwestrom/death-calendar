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

(deftest calendar-map
  (testing "Make sure calendar-map shows the correct number of days."
    (repeatedly 10
                (let [test-map (sut/calendar-map (date-generator)
                                                 (Period/ofDays (days-generator)))]
                  (is (= (:total test-map) (+ (:lived test-map)
                                              (:remaining test-map)))))))
  (testing "Give the user an indication that their input is invalid."
    (let [test-map-gen (fn [birth-day]
                         (sut/calendar-map birth-day (Period/ofDays (* 80 365.25))))
          alive-case   (test-map-gen (LocalDate/of 1960 1 1))
          dead-case    (test-map-gen (LocalDate/of 1930 1 1))]
      (is (nil? (:dead? alive-case)))
      (is (true? (:dead? dead-case))))))

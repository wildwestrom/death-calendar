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
  (:import (java.time LocalDate Period)
           (java.time.temporal ChronoUnit)))

(deftest death-day
  (testing "Given a birthday and lifespan return death-day."
    (are [death-day b-day lifespan] (= death-day (sut/death-day b-day lifespan))
      (LocalDate/of 2080 1 1)  (LocalDate/of 2000 1 1)  (Period/ofYears 80)
      (LocalDate/of 2098 8 15) (LocalDate/of 1998 8 15) (Period/ofYears 100)
      (LocalDate/of 2000 2 1)  (LocalDate/of 2000 1 1)  (Period/ofMonths 1)
      (LocalDate/of 2001 1 1)  (LocalDate/of 2000 1 1)  (Period/ofDays 366)
      (LocalDate/of 2000 2 29) (LocalDate/of 2000 1 1)  (Period/ofDays (+ 30 29)))))

(def ^:const long-human-life-years 100)

(defn alive-date-generator
  []
  (.plusYears (LocalDate/now)
              (rand-nth (range (+ 1 (- long-human-life-years)) long-human-life-years))))

(defn dead-date-generator
  []
  (.plusYears (LocalDate/now)
              (rand-nth (range (- (* 2 long-human-life-years))
                               (- (- long-human-life-years) 1)))))

(deftest calendar-map
  (dotimes [x 1000]
    (let [date (alive-date-generator)
          dead-date (dead-date-generator)
          num-of-years  long-human-life-years
          num-of-months (* 12 num-of-years)
          num-of-weeks  (* 52 num-of-years)]
      (println (str x ": " date))
      (testing "Give the user an indication that their input is invalid."
        (let [test-map-gen (fn [birth-day]
                             (sut/calendar-map birth-day (Period/ofWeeks num-of-weeks)))
              alive-case   (test-map-gen date)
              dead-case    (test-map-gen dead-date)]
          (is (nil? (:dead? alive-case)))
          (is (true? (:dead? dead-case)))))

      (testing "Has all required fields."
        (let [test-cal-map (sut/calendar-map dead-date (Period/ofWeeks num-of-weeks))]
          (is (some? (:lived test-cal-map)))
          (is (some? (:total test-cal-map)))
          (is (some? (:remaining test-cal-map)))
          (is (some? (:dead? test-cal-map)))))

      (testing "Extra flags for different units of time."
        (is (= (sut/calendar-map date (Period/ofWeeks num-of-weeks))
               (sut/calendar-map date (Period/ofWeeks num-of-weeks) :unit ChronoUnit/DAYS)))
        (are [num-of period unit]
             (= (:total (sut/calendar-map date (period num-of) :unit unit))
                num-of)
          num-of-years Period/ofYears ChronoUnit/YEARS
          num-of-months Period/ofMonths ChronoUnit/MONTHS
          num-of-weeks Period/ofWeeks ChronoUnit/WEEKS)))))

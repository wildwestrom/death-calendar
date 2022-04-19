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

(ns wildwestrom.death-calendar.core-test
  (:require [clojure.test :refer [deftest testing is are]]
            [wildwestrom.death-calendar.core :as sut]
            [clojure.test.check.generators :as gen]
            [clojure.test.check.properties :as prop]
            [clojure.test.check.clojure-test :refer [defspec]])
  (:import (java.time LocalDate Period)
           (java.time.temporal ChronoUnit)))

(def ^:const life-expectancy-years 100)

(deftest death-day
  (testing "Given a birthday and lifespan return death-day."
    (are [death-day b-day lifespan] (= death-day (sut/death-day b-day lifespan))
      (LocalDate/of 2080 1 1)  (LocalDate/of 2000 1 1)  (Period/ofYears 80)
      (LocalDate/of 2098 8 15) (LocalDate/of 1998 8 15) (Period/ofYears 100)
      (LocalDate/of 2000 2 1)  (LocalDate/of 2000 1 1)  (Period/ofMonths 1)
      (LocalDate/of 2001 1 1)  (LocalDate/of 2000 1 1)  (Period/ofDays 366)
      (LocalDate/of 2000 2 29) (LocalDate/of 2000 1 1)  (Period/ofDays (+ 30 29))
      (LocalDate/of 2095 2 28) (LocalDate/of 1996 2 29) (Period/ofYears 99))))

(def recent-date-generator
  "Generates a date from 1900-01-01 to a lifetime from now."
  (gen/fmap #(LocalDate/ofEpochDay %)
            (gen/choose -25567
                        (+ (int (* 365.25 life-expectancy-years))
                           (.toEpochDay (LocalDate/now))))))

(def alive-date-generator
  "Generates a birthday such that a person with that birthday
  is not older than a given life expectancy."
  (gen/fmap #(.plusYears (LocalDate/now) %)
            (gen/choose
             (+ 1 (- life-expectancy-years))
             life-expectancy-years)))

(def dead-date-generator
  "Generates a birthday such that a person with that birthday
  is older than a given life expectancy."
  (gen/fmap #(.plusYears (LocalDate/now) %)
            (gen/choose
             (- (* 2 life-expectancy-years))
             (- (- life-expectancy-years) 1))))

(defspec given-an-alive-date-return-true
  (prop/for-all [bday alive-date-generator]
                (true? (sut/alive?
                        bday
                        (Period/ofYears life-expectancy-years)))))

(defspec given-a-dead-date-dead-return-false
  (prop/for-all [bday dead-date-generator]
                (false? (sut/alive?
                         bday
                         (Period/ofYears life-expectancy-years)))))

(defspec ChronoUnit-DAYS-is-equal-to-no-ChronoUnit-specified
  (prop/for-all [date recent-date-generator
                 num-of-weeks (gen/fmap #(Period/ofWeeks %)
                                        (gen/fmap #(* 52 %)
                                                  (gen/choose -100 100)))]
                (is (= (sut/calendar-data date num-of-weeks)
                       (sut/calendar-data date num-of-weeks :unit ChronoUnit/DAYS)))))

let rateTimes = 8
let billableDaysPerMonth = 22.0

func dailyRateFrom(hourlyRate: Int) -> Double {
  return Double(hourlyRate * rateTimes)
}

func monthlyRateFrom(hourlyRate: Int, withDiscount discount: Double) -> Double {
  return Double(((dailyRateFrom(hourlyRate: hourlyRate) * billableDaysPerMonth) * discount).rounded(.down))
}

func workdaysIn(budget: Double, hourlyRate: Int, withDiscount discount: Double) -> Double {
  return Double((budget / (dailyRateFrom(hourlyRate: hourlyRate) * discount)).rounded(.down))
}

let lowestBaseCost = 25_000

func canIBuy(vehicle: String, price: Double, monthlyBudget: Double) -> String {
  let monthlyPrice = price / 60.0
  if (monthlyBudget >= monthlyPrice) {
    return "Yes! I'm getting a " + vehicle
  }
  else if (monthlyBudget * 1.1 >= monthlyPrice) {
    return "I'll have to be frugal if I want a " + vehicle
  }
  return "Darn! No " + vehicle + " for me"
}

func licenseType(numberOfWheels wheels: Int) -> String {
  switch wheels {
    case 2, 3:
      return "You will need a motorcycle license for your vehicle"
    case 4, 6:
      return "You will need an automobile license for your vehicle"
    case 18:
      return "You will need a commercial trucking license for your vehicle"
    default:
      return "We do not issue licenses for those types of vehicles"
  }
}

func registrationFee(msrp: Int, yearsOld: Int) -> Int {
  guard yearsOld < 10 else { return 25 }

  var cost: Int
    
  if (msrp > lowestBaseCost) {
    cost = msrp
  } else {
    cost = lowestBaseCost
  }

  return (cost - costReduction(cost: cost, yearsOld: yearsOld)) / 100
}

func costReduction(cost: Int, yearsOld: Int) -> Int {
  return Int(Double(cost) * 0.1 * Double(yearsOld))
}
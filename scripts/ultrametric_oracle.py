import json
import os

def k_to_ultrametric(k):
    expr = 11 * k + 3
    # v_2(expr)
    trailing = 0
    temp = expr
    while temp > 0 and (temp % 2 == 0):
        trailing += 1
        temp //= 2
    x = 5 + trailing
    unit = temp
    return x, unit

def step_u(x, unit):
    if x < 4:
        return {"outcome": "non_integral"}
    image_x = x - 4
    image_unit = unit * 27
    if image_x < 5:
        return {"outcome": "exact_leaves_q1", "next_x": image_x, "next_unit": image_unit}
    else:
        return {"outcome": "based_return", "next_x": image_x, "next_unit": image_unit}

def step_v_resonant(x, unit):
    if x != 6:
        return {"outcome": "non_integral"}
    expr = 729 * unit + 87
    gamma = 0
    temp = expr
    while temp > 0 and (temp % 2 == 0):
        gamma += 1
        temp //= 2
    
    if gamma < 3:
        return {"outcome": "non_integral"}
    elif gamma == 3:
        return {"outcome": "integral_even_outside_q1", "next_x": 0}
    else:
        image_x = gamma - 3
        image_unit = temp
        if image_x < 5:
            return {"outcome": "exact_leaves_q1", "next_x": image_x, "next_unit": image_unit}
        else:
            return {"outcome": "based_return", "next_x": image_x, "next_unit": image_unit}

def main():
    # Test k = 7 (n = 231) => u returns next_k = 12 (n' = 391)
    x7, u7 = k_to_ultrametric(7)
    out_u = step_u(x7, u7)
    assert out_u["outcome"] == "based_return"
    x12_expected, u12_expected = k_to_ultrametric(12)
    assert out_u["next_x"] == x12_expected and out_u["next_unit"] == u12_expected

    # Test k = 61 (n = 1959) => v returns next_k = 87 (n' = 2791)
    x61, u61 = k_to_ultrametric(61)
    out_v = step_v_resonant(x61, u61)
    assert out_v["outcome"] == "based_return"
    x87_expected, u87_expected = k_to_ultrametric(87)
    assert out_v["next_x"] == x87_expected and out_v["next_unit"] == u87_expected

    report = {
        "schema_version": "phase73b_2_verification_report_v1",
        "transitions": [
            {
                "schema_version": "ultrametric_state_transition_v1",
                "valuation_word": [1, 1, 2],
                "starting_k": "7",
                "start_x": x7,
                "start_unit": str(u7),
                "outcome_type": "based_return",
                "next_x": out_u["next_x"],
                "next_unit": str(out_u["next_unit"])
            },
            {
                "schema_version": "ultrametric_state_transition_v1",
                "valuation_word": [1, 1, 2, 1, 2, 2],
                "starting_k": "61",
                "start_x": x61,
                "start_unit": str(u61),
                "outcome_type": "based_return",
                "next_x": out_v["next_x"],
                "next_unit": str(out_v["next_unit"])
            }
        ],
        "all_commuting_diagrams_verified": True
    }

    os.makedirs("artifacts/phase73b_2", exist_ok=True)
    with open("artifacts/phase73b_2/python_ultrametric_results.json", "w") as f:
        json.dump(report, f, indent=2)

    with open("artifacts/phase73b_2/phase73b_2_verification_report.json", "w") as f:
        json.dump(report, f, indent=2)

    print("Independent Python Ultrametric Oracle PASSED successfully!")

if __name__ == "__main__":
    main()

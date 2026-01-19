/**
 * Mathematical utilities for orthogonal array construction validation
 */

/**
 * Check if a number is prime
 */
export function isPrime(n: number): boolean {
  if (n < 2) return false;
  if (n === 2) return true;
  if (n % 2 === 0) return false;
  for (let i = 3; i * i <= n; i += 2) {
    if (n % i === 0) return false;
  }
  return true;
}

/**
 * Check if a number is a prime power (p^k where p is prime and k >= 1)
 * Returns { valid: true, base, exponent } if it's a prime power
 * Returns { valid: false } otherwise
 */
export function isPrimePower(n: number): { valid: boolean; base?: number; exponent?: number } {
  if (n < 2) return { valid: false };

  // Check if it's a prime itself (p^1)
  if (isPrime(n)) {
    return { valid: true, base: n, exponent: 1 };
  }

  // Check if it's a power of a prime
  for (let base = 2; base * base <= n; base++) {
    if (!isPrime(base)) continue;

    let power = base;
    let exponent = 1;
    while (power < n) {
      power *= base;
      exponent++;
    }
    if (power === n) {
      return { valid: true, base, exponent };
    }
  }

  return { valid: false };
}

/**
 * Get all prime powers up to a limit
 */
export function getPrimePowersUpTo(limit: number): number[] {
  const result: number[] = [];
  for (let n = 2; n <= limit; n++) {
    if (isPrimePower(n).valid) {
      result.push(n);
    }
  }
  return result;
}

/**
 * Check if a number is a multiple of 4 (for Hadamard construction)
 */
export function isMultipleOf4(n: number): boolean {
  return n > 0 && n % 4 === 0;
}

/**
 * Check if a number is an odd prime (for Addelman-Kempthorne construction)
 */
export function isOddPrime(n: number): boolean {
  return n > 2 && isPrime(n);
}

/**
 * Format a prime power for display (e.g., "2³ = 8")
 */
export function formatPrimePower(n: number): string {
  const result = isPrimePower(n);
  if (!result.valid || !result.base || !result.exponent) {
    return String(n);
  }
  if (result.exponent === 1) {
    return `${n} (prime)`;
  }
  // Use Unicode superscripts for exponents
  const superscripts: Record<number, string> = {
    0: '⁰', 1: '¹', 2: '²', 3: '³', 4: '⁴', 5: '⁵', 6: '⁶', 7: '⁷', 8: '⁸', 9: '⁹'
  };
  const expStr = String(result.exponent).split('').map(d => superscripts[parseInt(d)] || d).join('');
  return `${result.base}${expStr} = ${n}`;
}

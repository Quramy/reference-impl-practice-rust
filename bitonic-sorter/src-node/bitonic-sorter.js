/**
 *
 * @param x - Array. 要素数は2のべき乗でなければならない
 * @param up - boolean. trueの場合は昇順
 *
 */
function sort(x, up) {
  if (x.length <= 1) {
    // 要素数が1になったらおしまい
    return x;
  } else {
    // Step a
    // 前半を昇順, 後半を降順でソートする
    const midPoint = ~~(x.length / 2);
    const first = sort(x.slice(0, midPoint), true);
    const second = sort(x.slice(midPoint), false);

    // Step b
    const x1 = [...first, ...second];
    return subSort(x1, up);
  }
}

function subSort(x, up) {
  if (x.length <= 1) {
    return x;
  } else {
    // Step 2a
    // 要素数nのバイトニック列の要素をn/2要素おきに比較し、
    // upで指定された順序（昇順または降順）になるように交換する
    compareAndSwap(x, up);

    // Step 2b
    const midPoint = ~~(x.length / 2);
    const first = subSort(x.slice(0, midPoint), up);
    const second = subSort(x.slice(midPoint), up);

    // Step 2c
    return [...first, ...second];

  }
}

function compareAndSwap(x, up) {
  const midPoint = ~~(x.length / 2);
  let i;
  for (i = 0; i < midPoint; i++) {
    if ((x[i] > x[midPoint + i]) === up) {
      [x[i], x[midPoint + i]] = [x[midPoint + i], x[i]];
    }
  }
}

module.exports = sort;

set -e
for S in 123 777; do
  echo SEED $S starting
  sed -i.bak "s/^SEED = 42$/SEED = $S/" train_pub.py
  caffeinate -i python3 -u train_pub.py 2>&1 | tee train_log_tiers_seed$S.txt
  mv train_pub.py.bak train_pub.py
  echo SEED $S done
done
echo ALL SEEDS DONE

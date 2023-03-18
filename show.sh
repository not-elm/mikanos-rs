set -u

usage() {
    echo "
usage:
    $0 [-x]
" >&2
}

is_hex=false
# オプション解析
while getopts ab:ch OPT; do
    case $OPT in
    h)
       is_hex=true
      ;;
    *)
      ;;
    esac
done


NUM=$1

[ $is_hex ] && BASE=16 || BASE=10

HEX=$(echo "obase=2; ibase=$BASE; $NUM" |bc )
echo "0b$HEX"

NUMERIC=$(echo "obase=10; ibase=$BASE; $NUM" |bc )
echo "$NUMERIC"

HEX=$(echo "obase=16; ibase=$BASE; $NUM" |bc )
echo "0x$HEX"
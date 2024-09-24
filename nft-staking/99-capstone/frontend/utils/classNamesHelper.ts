const cn = (...args: Array<undefined | null | boolean | string>): string => {
	return args.filter((arg) => typeof arg === "string").join(" ");
};

export default cn;

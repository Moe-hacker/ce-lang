fmt:
	sed -i "s/:</_CE_PAN/g" t.ce
	sed -i "s/:>/_CE_DFT/g" t.ce
	clang-format -i --assume-filename=t.c t.ce 
	sed -i "s/_CE_PAN/:</g" t.ce
	sed -i "s/_CE_DFT/:>/g" t.ce
	clang-format -i --assume-filename=t.c t.hce 
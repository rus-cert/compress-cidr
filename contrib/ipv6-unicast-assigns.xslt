<?xml version="1.0" encoding="UTF-8"?>
<!-- saxonb-xslt http://www.iana.org/assignments/ipv6-unicast-address-assignments/ipv6-unicast-address-assignments.xml unicast-assigns.xslt -->
<xsl:stylesheet version="2.0"
        xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
        xmlns:iana="http://www.iana.org/assignments"
>

        <xsl:output method="text" omit-xml-declaration="yes" encoding="UTF-8"/>

        <xsl:variable name='nl'><xsl:text>&#xa;</xsl:text></xsl:variable>

        <xsl:template match="/">
                <xsl:for-each select="iana:registry[@id='ipv6-unicast-address-assignments']/iana:record">
                        <xsl:if test="iana:status = 'ALLOCATED'">
                                <xsl:value-of select="concat(iana:prefix,$nl)" />
                        </xsl:if>
                </xsl:for-each>
        </xsl:template>
</xsl:stylesheet>

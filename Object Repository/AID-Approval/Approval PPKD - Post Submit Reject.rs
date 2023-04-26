<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Approval PPKD - Post Submit Reject</name>
   <tag></tag>
   <elementGuidId>40c48dfa-4003-488c-93fb-3a8cb22b0930</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n  \&quot;DocumentNo\&quot;: \&quot;0006571/PPID/03/22\&quot;,\r\n  \&quot;UserID\&quot;: \&quot;20010095\&quot;,\r\n  \&quot;KomisiAsuransi\&quot;: 1, \r\n  \&quot;RefundRate\&quot;: 0, \r\n  \&quot;InaturaProvisi\&quot;: 0,\r\n  \&quot;InaturaProgram\&quot;: 1,\r\n  \&quot;KomisiAdmin\&quot;: 0,\r\n  \&quot;Keterangan\&quot;: \&quot;Tolong Revisi \&quot;\r\n}\r\n// komisi asuransi - komisi admin di isi 1 bila di rejct, kalau tidak di isi 0\r\n//conthonya di atas itu bm reject komisi asuransi dan inatrua program&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic QkFTSUNfQVVUSDpNb2JpbGUxMjMr</value>
      <webElementGuid>8d2dd380-2559-4020-811a-3faeff2bbdf7</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://192.168.1.254:8089/api/ApprovalPermohonanPembayaranKomisiDealer/Reject</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
